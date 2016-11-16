import javax.imageio.ImageIO;
import javax.swing.*;
import javax.swing.event.ListSelectionEvent;
import javax.swing.event.ListSelectionListener;
import javax.swing.filechooser.FileFilter;
import javax.swing.filechooser.FileNameExtensionFilter;
import java.awt.*;
import java.awt.event.MouseAdapter;
import java.awt.event.MouseEvent;
import java.awt.image.BufferedImage;
import java.io.File;
import java.util.ArrayList;

public class FormMain extends JFrame {
	private JButton button1;
	private JPanel panel;
	private File lastDirOpen = new File("./");
	private File lastDirSave = new File("./");

	private ArrayList<BufferContainer> img_data = new ArrayList<>();

	public FormMain() {
		button1.addMouseListener(new MouseAdapter() {
			@Override
			public void mouseClicked(MouseEvent e) {
				JFileChooser chooser = new JFileChooser();
				chooser.setFileSelectionMode(JFileChooser.FILES_AND_DIRECTORIES);
				chooser.setMultiSelectionEnabled(true);
				chooser.setCurrentDirectory(lastDirOpen);
				chooser.setFileFilter(new FileFilter() {
					String[] approvedExt = {"png", "jpg", "jpeg"};

					@Override
					public boolean accept(File f) {
						if (f.isDirectory())
							return true;
						for (String str : approvedExt) {
							if (f.getName().matches(".*\\." + str + "$")) {
								return true;
							}
						}
						return false;
					}

					@Override
					public String getDescription() {
						return null;
					}
				});
				if (chooser.showOpenDialog(panel) == JFileChooser.APPROVE_OPTION) {
					lastDirOpen=chooser.getCurrentDirectory();
					parseImages(chooser.getSelectedFiles());
				}
			}
		});


	}

	private void parseImages(File[] images) {
		for (int i = 0; i < images.length - 1; i += 2) {
			try {
				BufferedImage imgA = ImageIO.read(images[i]);
				BufferedImage imgB = ImageIO.read(images[i + 1]);
				BufferedImage imgBuffer = new BufferedImage(imgA.getWidth(), imgA.getHeight() + imgB.getHeight(), BufferedImage.TYPE_INT_RGB);
				JPanel gui = new JPanel(new BorderLayout(3, 3));

				Graphics2D g2dColumn = imgBuffer.createGraphics();
				g2dColumn.drawImage(imgA, 0, 0, null);
				// start this one at 'height' down the final image
				g2dColumn.drawImage(imgB, 0, imgA.getHeight(), null);
				String name = images[i].getName().substring(0, 5) + images[i + 1].getName().substring(0, 5);
				img_data.add(new BufferContainer(imgBuffer, name));

				gui.add(new JLabel(new ImageIcon(imgBuffer)), BorderLayout.CENTER);

//				JOptionPane.showMessageDialog(null, gui);
				int resp = JOptionPane.showOptionDialog(null,gui,"Result",JOptionPane.OK_OPTION,JOptionPane.NO_OPTION,null,new String[]{"Save","Edit"},JOptionPane.OK_OPTION);

				if (resp==JOptionPane.OK_OPTION) {
					JFileChooser chooser = new JFileChooser();
					FileFilter filter = new FileNameExtensionFilter("Image Files",new String[]{"png","jpg","jpeg"});
					chooser.addChoosableFileFilter(filter);
					chooser.setFileFilter(filter);
					chooser.setCurrentDirectory(lastDirSave);
					chooser.setSelectedFile(new File(name+".png"));
					int result = chooser.showSaveDialog(null);
					if (result==JFileChooser.APPROVE_OPTION) {
						lastDirSave=chooser.getCurrentDirectory();
						ImageIO.write(imgBuffer, "png", chooser.getSelectedFile());
					}
				}
			} catch (Exception e) {
				e.printStackTrace();
			}
		}
	}

	private class BufferContainer {
		private BufferedImage buffer;
		private String name;

		public BufferContainer(BufferedImage img, String title) {
			buffer = img;
			name = title;
		}

		@Override
		public String toString() {
			return name;
		}
	}

	public static FormMain instance;

	public static void main(String args[]) {
		JFrame frame = new JFrame("FormMain");
		instance = new FormMain();
		frame.setContentPane(instance.panel);
		frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
		frame.pack();
		instance.setLocationRelativeTo(null);
		frame.setVisible(true);
	}
}
