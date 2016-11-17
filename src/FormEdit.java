import javax.imageio.ImageIO;
import javax.swing.*;
import java.awt.*;
import java.awt.event.MouseAdapter;
import java.awt.event.MouseEvent;
import java.awt.geom.AffineTransform;
import java.awt.image.BufferedImage;
import java.io.File;
import java.util.*;

public class FormEdit {
    private JPanel IMGPanel;
    private JPanel panel;
    private JButton OKButton;
    private JButton PREVButton;
    private JButton NEXTButton;
    private JButton FINISHButton;

    private ArrayList<ImagePair> imagePairs;
    private ListIterator<ImagePair> iter;

    public FormEdit() {
        buildBuffers();
        PREVButton.addMouseListener(new MouseAdapter() {
            @Override
            public void mouseClicked(MouseEvent e) {
                onPrev();
            }
        });
        NEXTButton.addMouseListener(new MouseAdapter() {
            @Override
            public void mouseClicked(MouseEvent e) {
                onNext();
            }
        });
        FINISHButton.addMouseListener(new MouseAdapter() {
            @Override
            public void mouseClicked(MouseEvent e) {
                onFINISH();
            }
        });
        onNext();
    }


    private void onPrev() {
        updateButtons();
        if (iter.hasPrevious()) {
            ImagePair pair = iter.previous();
            displayPair(pair);

        }
        updateButtons();
    }

    private void onNext() {
        updateButtons();
        if (iter.hasNext()) {
            ImagePair pair = iter.next();
            displayPair(pair);
        }
        updateButtons();
    }

    private void displayPair(ImagePair pair) {
        if (pair.lastBuffer == null)
            return;
        IMGPanel.removeAll();
        IMGPanel.add(new JLabel(new ImageIcon(pair.lastBuffer.getScaledInstance(400, 400, Image.SCALE_SMOOTH))), BorderLayout.CENTER);
        IMGPanel.revalidate();
        IMGPanel.repaint();
    }

    private void onFINISH() {
        for (ImagePair pair : imagePairs) {
            File output = new File(PhotoLayout.getSavePath().getAbsoluteFile() + File.separator + pair.fileName + ".png");
            try {
                ImageIO.write(pair.lastBuffer, "png", output);
            } catch (Exception e) {
                System.out.println("Error saving file:");
                e.printStackTrace();
            }
        }
        PhotoLayout.form_prompt.setVisible(true);
        PhotoLayout.clearImages();
        PhotoLayout.form_edit.dispose();

    }

    private void updateButtons() {
        PREVButton.setEnabled(iter.hasPrevious());
        NEXTButton.setEnabled(iter.hasNext());
    }

    private void buildBuffers() {
        imagePairs = new ArrayList<>();
        ArrayList<File> images = PhotoLayout.getImages();
        for (int index = 0; index < images.size() - 1; index += 2) {
            ImagePair imagePair = new ImagePair(images.get(index), images.get(index + 1));
            try {
                BufferedImage imgA = ImageIO.read(imagePair.fileA);
                BufferedImage imgB = ImageIO.read(imagePair.fileB);

                int width = greaterOf(greaterOf(imgA.getHeight(), imgA.getWidth()), greaterOf(imgB.getHeight(), imgB.getWidth()));
                int height = lesserOf(imgA.getHeight(), imgA.getWidth()) + lesserOf(imgB.getHeight(), imgB.getWidth());

                BufferedImage imgBuffer = new BufferedImage(width, height, BufferedImage.TYPE_INT_ARGB);


                AffineTransform topShift = new AffineTransform();
                if (imgA.getHeight() > imgA.getWidth()) {
                    topShift.translate(imgA.getHeight() / 2,imgA.getWidth() / 2);
                    topShift.rotate(Math.PI / 2);
                    topShift.translate(-imgA.getWidth() / 2,-imgA.getHeight() / 2);
                }
                AffineTransform bottomShift = new AffineTransform();
                bottomShift.translate(0, lesserOf(imgA.getHeight(),imgA.getWidth()));
                if (imgB.getHeight() > imgB.getWidth()){
                    bottomShift.translate(imgB.getHeight() / 2,imgB.getWidth() / 2);
                    bottomShift.rotate(Math.PI / 2);
                    bottomShift.translate(-imgB.getWidth() / 2,-imgB.getHeight() / 2);
                }
                Graphics2D graphics = imgBuffer.createGraphics();
                graphics.setBackground(Color.WHITE);
                graphics.drawImage(imgA, topShift, null);
                graphics.drawImage(imgB, bottomShift, null);
                graphics.dispose();


                String name = imagePair.fileA.getName().substring(0, 5) + imagePair.fileB.getName().substring(0, 5);
                imagePair.fileName = name;
                imagePair.lastBuffer = imgBuffer;
            } catch (Exception e) {
                e.printStackTrace();
            }
            imagePairs.add(imagePair);
        }
        iter = imagePairs.listIterator();
    }

    private int lesserOf(int a, int b) {
        return a < b ? a : b;
    }

    private int greaterOf(int a, int b) {
        return a > b ? a : b;
    }

    public static void main(String[] args) {
        JFrame instance = new JFrame("FormEdit");
        PhotoLayout.form_edit = instance;
        PhotoLayout.inst_edit = new FormEdit();
        instance.setContentPane(PhotoLayout.inst_edit.panel);
        instance.setDefaultCloseOperation(JFrame.DISPOSE_ON_CLOSE);
        instance.pack();
        instance.setVisible(true);
    }

    private class ImagePair {
        private final File fileA;
        private final File fileB;
        private String fileName;
        private BufferedImage lastBuffer;


        public ImagePair(File fileA, File fileB) {
            this.fileA = fileA;
            this.fileB = fileB;
        }

    }
}
