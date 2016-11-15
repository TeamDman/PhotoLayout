import javax.swing.*;
import javax.swing.event.ListSelectionEvent;
import javax.swing.event.ListSelectionListener;
import javax.swing.filechooser.FileFilter;
import java.awt.event.MouseAdapter;
import java.awt.event.MouseEvent;
import java.io.File;
import java.util.ArrayList;

public class FormMain extends JFrame {
	private JButton button1;
	private JLabel img_left;
	private JPanel panel;
	private JList img_list;
	private JLabel img_preview;
	private File[] files;
	public FormMain() {
		button1.addMouseListener(new MouseAdapter() {
			@Override
			public void mouseClicked(MouseEvent e) {
				JFileChooser chooser = new JFileChooser();
				chooser.setFileSelectionMode(JFileChooser.FILES_AND_DIRECTORIES);
				chooser.setMultiSelectionEnabled(true);
//				chooser.setCurrentDirectory(new File(System.getProperty("user.home")+File.separator+"Pictures"));
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
					files = chooser.getSelectedFiles();
					img_list.setListData(files);
					for (File file : files) {
						System.out.println(file.getName());
						img_preview.setIcon(new ImageIcon(file.getAbsolutePath()));
					}
				}
			}
		});
		img_list.getSelectionModel().addListSelectionListener(new ListSelectionListener() {
			@Override
			public void valueChanged(ListSelectionEvent e) {
				img_preview.setIcon(new ImageIcon(files[e.getFirstIndex()].getAbsolutePath()));
			}
		});

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
