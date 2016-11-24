import javax.imageio.ImageIO;
import javax.swing.*;
import java.awt.*;
import java.awt.geom.AffineTransform;
import java.awt.image.BufferedImage;
import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;

public class PhotoLayout {
    private static ArrayList<File> images = new ArrayList<>();
    private static File path_save;

    public static FormPrompt inst_prompt;
    public static JFrame form_prompt;

    public static File getSavePath() {
        return path_save;
    }

    public static void setSavePath(File path_save) {
        PhotoLayout.path_save = path_save;
    }

    public static ArrayList<File> getImages() {
        return images;
    }

    public static void addImages(File[] toAdd) {
        images.addAll(Arrays.asList(toAdd));
    }

    public static void clearImages() {
        images.clear();
    }

    public static void main(String[] args) {
        FormPrompt.main(args);
    }

    public static void beginParse(String splitText) {
        if (path_save == null) {
            JPanel gui = new JPanel(new BorderLayout(3, 3));
            JLabel label = new JLabel();
            label.setText("You did not set a save path!");
            gui.add(label);
            JOptionPane.showMessageDialog(null, gui);
        } else {
            stitchImages(splitText);
        }
    }

    private static void stitchImages(String splitText) {
        Thread thread = new Thread() {
            @Override
            public void run() {
                for (int index = 0; index < images.size() - 1; index += 2) {
                    ImagePair imagePair = new ImagePair(images.get(index), images.get(index + 1));
                    try {
                        BufferedImage imgA = ImageIO.read(imagePair.fileA);
                        BufferedImage imgB = ImageIO.read(imagePair.fileB);

                        BufferedImage imgBuffer = new BufferedImage(1200, 1800, BufferedImage.TYPE_INT_RGB);

                        Graphics2D graphics = imgBuffer.createGraphics();
                        graphics.setBackground(Color.WHITE);

                        drawCentered(graphics, imgA);
                        graphics.translate(0, 900);
                        drawCentered(graphics, imgB);
                        graphics.dispose();

                        String name = buildName(imagePair.fileA,imagePair.fileB,splitText);
                        try {
                            File output = new File(PhotoLayout.getSavePath().getAbsoluteFile() + File.separator + name + ".jpg");
                            ImageIO.write(imgBuffer, "JPEG", output);
                        } catch (Exception e) {
                            JOptionPane.showMessageDialog(null,"Files failed to save, check that the split text is valid");
                            break;
                        }


                        if (index > 0) {
                            int prog = index * 100 / images.size();
                            inst_prompt.updateProgress(prog);
                        }
                    } catch (Exception e) {
                        e.printStackTrace();
                    }
                }
                inst_prompt.updateProgress(0);
            }
        };
        thread.start();
    }

    private static String buildName(File A, File B,String splitText) {
        return chopExtension(A.getName())+ splitText + chopExtension(B.getName());
    }

    private static String chopExtension(String name) {
        int i = name.lastIndexOf(".");
        return i==-1?name:name.substring(0,i);
    }

    private static void drawCentered(Graphics2D graphics, BufferedImage image) {
        AffineTransform trans = new AffineTransform();
        int width = image.getWidth();
        int height = image.getHeight();
        double scale;
        if (height > width) {
            scale = getScale(height, width);
            trans.setToScale(scale, scale);
            trans.translate(height / 2, width / 2);
            trans.rotate(Math.PI / 2);
            trans.translate(-image.getWidth() / 2, -height / 2);
            int temp = width;
            width = height;
            height = temp;
        } else {
            scale = getScale(image.getWidth(), height);
            trans.setToScale(scale, scale);
        }
        
        width = (int) (width * scale);
        height = (int) (height * scale);
        int dx = (width - 1200) / 2;
        int dy = (height - 900) / 2;
        graphics.setClip(0,0,1200,900);
        graphics.translate(-dx, -dy);
        graphics.drawImage(image, trans, null);
        graphics.translate(dx, dy);
    }

    private static double getScale(int width, int height) {
        double scale;
        scale = 1200.0 / width;
        if (height * scale < 900.0)
            scale = 900.0 / height;
        return scale;
    }

    private static class ImagePair {
        private final File fileA;
        private final File fileB;

        public ImagePair(File fileA, File fileB) {
            this.fileA = fileA;
            this.fileB = fileB;
        }

    }
}
