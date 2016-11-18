import javax.imageio.ImageIO;
import javax.swing.*;
import java.awt.*;
import java.awt.geom.AffineTransform;
import java.awt.image.BufferedImage;
import java.io.File;
import java.util.ArrayList;
import java.util.Arrays;

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

	public static void beginParse() {
		if (path_save == null) {
			JPanel gui = new JPanel(new BorderLayout(3, 3));
			JLabel label = new JLabel();
			label.setText("You did not set a save path!");
			gui.add(label);
			JOptionPane.showMessageDialog(null, gui);
		} else {
			stitchImages();
		}
	}

	private static void stitchImages() {
		Thread thread = new Thread() {
			@Override
			public void run() {
				for (int index = 0; index < images.size() - 1; index += 2) {
					ImagePair imagePair = new ImagePair(images.get(index), images.get(index + 1));
					try {
						BufferedImage imgA = ImageIO.read(imagePair.fileA);
						BufferedImage imgB = ImageIO.read(imagePair.fileB);

						BufferedImage imgBuffer = new BufferedImage(1200, 1800, BufferedImage.TYPE_INT_ARGB);

						AffineTransform topShift = buildTransform(imgA);
						AffineTransform bottomShift = buildTransform(imgB);

						Graphics2D graphics = imgBuffer.createGraphics();
						graphics.setBackground(Color.WHITE);
						graphics.drawImage(imgA, topShift, null);
						graphics.translate(0,900);
						graphics.drawImage(imgB, bottomShift, null);
						graphics.dispose();

						String name = imagePair.fileA.getName().substring(0, 5) + imagePair.fileB.getName().substring(0, 5);
						File output = new File(PhotoLayout.getSavePath().getAbsoluteFile() + File.separator + name + ".png");
						ImageIO.write(imgBuffer, "png", output);
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

	private static AffineTransform buildTransform(BufferedImage image) {
		AffineTransform trans = new AffineTransform();
		if (image.getHeight() > image.getWidth()) {
			trans.setToScale(1.0 / (image.getHeight() / 1200.0), 1.0 / (image.getWidth() / 900.0));
			trans.translate(image.getHeight() / 2, image.getWidth() / 2);
			trans.rotate(Math.PI / 2);
			trans.translate(-image.getWidth() / 2, -image.getHeight() / 2);
		} else {
			trans.setToScale(1200.0 / image.getWidth(),900.0 / image.getHeight());
		}
		return trans;
	}

	private static int lesserOf(int a, int b) {
		return a < b ? a : b;
	}

	private static int greaterOf(int a, int b) {
		return a > b ? a : b;
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
