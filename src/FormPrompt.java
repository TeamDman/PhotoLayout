import javax.swing.*;
import javax.swing.filechooser.FileFilter;
import java.awt.*;
import java.awt.event.ActionListener;
import java.awt.event.MouseAdapter;
import java.awt.event.MouseEvent;
import java.io.File;
import java.util.*;

/**
 * Created by s401321 on 16/11/2016.
 */
public class FormPrompt {
    private JTextField path_save_field;
    private JButton path_save_prompt;
    private JButton beginButton;
    private JPanel panel;
    private JList filelist;
    private JButton addFilesButton;
    private JButton CLEARButton;
    private JProgressBar prog;
    private JTextField nameField;

    private File path_open_dir = new File("./");
    private File path_save_dir = new File("./");
    private ArrayList<String> extensions = new ArrayList<>(Arrays.asList("png", "jpg", "jpeg"));

    public static void main(String[] args) {
        JFrame instance = new JFrame("FormPrompt");
        PhotoLayout.inst_prompt = new FormPrompt();
        PhotoLayout.form_prompt = instance;
        instance.setContentPane(PhotoLayout.inst_prompt.panel);
        instance.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
//        instance.setMinimumSize(new Dimension(500,500));
        instance.pack();
        instance.setVisible(true);
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
            path_open_dir =chooser.getCurrentDirectory();
            PhotoLayout.addImages(chooser.getSelectedFiles());
            updateList();
        }
    }

    public void updateList() {
        filelist.setListData(PhotoLayout.getImages().toArray());
    }

    public void updateProgress(int progress) {
		prog.setValue(progress);
	}

    public void clearList() {
        PhotoLayout.clearImages();
        updateList();
    }

    public FormPrompt() {
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
                clearList();
            }
        });

    }
}
