import javax.swing.*;
import javax.swing.filechooser.FileFilter;
import java.awt.event.MouseAdapter;
import java.awt.event.MouseEvent;
import java.io.File;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;

/**
 * Created by s401321 on 16/11/2016.
 */
public class FormPrompt {
	private JButton           CLEARButton;
	private JButton           addFilesButton;
	private JButton           beginButton;
	private ArrayList<String> extensions    = new ArrayList<>(Arrays.asList("png", "jpg", "jpeg"));
	private JList             filelist;
	private JTextField        nameField;
	private JButton           openDestinationButton;
	private JPanel            panel;
	private File              path_open_dir = new File("./");
	private File              path_save_dir = new File("./");
	private JTextField        path_save_field;
	private JButton           path_save_prompt;
	private JProgressBar      prog;

	private FormPrompt() {
		path_save_prompt.addMouseListener(new MouseAdapter() {
			@Override
			public void mouseClicked(MouseEvent e) {
				getSaveDir();
			}
		});
		addFilesButton.addMouseListener(new MouseAdapter() {
			@Override
			public void mouseClicked(MouseEvent e) {
				getFiles();
			}
		});
		beginButton.addMouseListener(new MouseAdapter() {
			@Override
			public void mouseClicked(MouseEvent e) {
				PhotoLayout.beginParse(nameField.getText());
			}
		});
		CLEARButton.addMouseListener(new MouseAdapter() {
			@Override
			public void mouseClicked(MouseEvent e) {
				int confirm = JOptionPane.showConfirmDialog(panel, "Are you sure you want to clear the file list?", "Confirm Remove", JOptionPane.YES_NO_OPTION, JOptionPane.WARNING_MESSAGE);
				if (confirm == JOptionPane.YES_OPTION) {
					clearList();
				}
			}
		});
		openDestinationButton.addMouseListener(new MouseAdapter() {
			@Override
			public void mouseClicked(MouseEvent e) {
				try {
					java.awt.Desktop.getDesktop().open(PhotoLayout.getSavePath());
				} catch (IOException ex) {
					ex.printStackTrace();
				}
			}
		});
		new FileDrop(panel, files -> {
			PhotoLayout.addImages(files);
			updateList();
		});
		filelist.addMouseListener(new MouseAdapter() {
			@Override
			public void mouseReleased(MouseEvent e) {
				if (e.getButton() == MouseEvent.BUTTON3) {
					int confirm = JOptionPane.showConfirmDialog(panel, "Are you sure you want to remove the selected elements?", "Confirm Remove", JOptionPane.YES_NO_OPTION, JOptionPane.WARNING_MESSAGE);
					if (confirm == JOptionPane.YES_OPTION) {
						PhotoLayout.removeImages(filelist.getSelectedValues());
						updateList();
					}
				}
			}
		});
	}

	private void getSaveDir() {
		JFileChooser chooser = new JFileChooser();
		chooser.setFileSelectionMode(JFileChooser.DIRECTORIES_ONLY);
		chooser.setMultiSelectionEnabled(true);
		chooser.setCurrentDirectory(path_save_dir);
		if (chooser.showOpenDialog(panel) == JFileChooser.APPROVE_OPTION) {
			path_save_dir = chooser.getSelectedFile();
			path_save_field.setText(path_save_dir.getAbsolutePath());
			PhotoLayout.setSavePath(path_save_dir);
		}
	}

	private void getFiles() {
		JFileChooser chooser = new JFileChooser();
		chooser.setFileSelectionMode(JFileChooser.FILES_ONLY);
		chooser.setMultiSelectionEnabled(true);
		chooser.setCurrentDirectory(path_open_dir);
		chooser.setFileFilter(new FileFilter() {
			@Override
			public boolean accept(File f) {
				if (f.isDirectory())
					return true;
				for (String str : extensions) {
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
			path_open_dir = chooser.getCurrentDirectory();
			PhotoLayout.addImages(chooser.getSelectedFiles());
			updateList();
		}
	}

	public void updateList() {
		filelist.setListData(PhotoLayout.getImages().toArray());
	}

	public void clearList() {
		PhotoLayout.clearImages();
		updateList();
	}

	public static void main(String[] args) {
		try {
			UIManager.setLookAndFeel(UIManager.getSystemLookAndFeelClassName());
		} catch (Exception e) {
			e.printStackTrace();
		}
		JFrame instance = new JFrame("FormPrompt");
		instance.setTitle("PhotoLayout Application");
		PhotoLayout.inst_prompt = new FormPrompt();
		PhotoLayout.form_prompt = instance;
		instance.setContentPane(PhotoLayout.inst_prompt.panel);
		instance.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
		instance.pack();
		instance.setVisible(true);
	}

	public void updateProgress(int progress) {
		prog.setValue(progress);
	}
}
