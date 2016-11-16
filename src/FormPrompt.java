import javax.swing.*;
import javax.swing.filechooser.FileFilter;
import java.awt.*;
import java.awt.event.MouseAdapter;
import java.awt.event.MouseEvent;
import java.io.File;
import java.util.*;
import java.util.stream.Collectors;

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
    private static FormPrompt instance;

    private File path_open_dir = new File("./");
    private File path_save_dir = new File("./");
    private HashSet<File> selectedFiles = new HashSet<>();
    private ArrayList<String> extensions = new ArrayList<>(Arrays.asList("png", "jpg", "jpeg"));

    public static void main(String[] args) {
        instance = new FormPrompt();
        JFrame frame = new JFrame("FormPrompt");
        frame.setContentPane(instance.panel);
        frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        frame.setMinimumSize(new Dimension(500,500));
        frame.pack();
        frame.setVisible(true);
    }

    private void getSaveDir() {
        JFileChooser chooser = new JFileChooser();
        chooser.setFileSelectionMode(JFileChooser.DIRECTORIES_ONLY);
        chooser.setMultiSelectionEnabled(true);
        chooser.setCurrentDirectory(path_save_dir);
        if (chooser.showOpenDialog(panel) == JFileChooser.APPROVE_OPTION) {
            path_save_dir = chooser.getSelectedFile();
            path_save_field.setText(path_save_dir.getAbsolutePath());
        }
    }

    private void getFiles() {JFileChooser chooser = new JFileChooser();
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
            selectedFiles.addAll(Arrays.stream(chooser.getSelectedFiles()).collect(Collectors.toSet()));
            filelist.setListData(selectedFiles.toArray());
        }
    }



    public FormPrompt() {
        path_save_field.setText(path_save_dir.getPath());
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
    }
}
