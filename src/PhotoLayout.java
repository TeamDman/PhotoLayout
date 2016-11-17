import javax.swing.*;
import java.awt.*;
import java.io.File;
import java.util.ArrayList;
import java.util.Arrays;

public class PhotoLayout {
	private static ArrayList<File> images = new ArrayList<>();
	private static File path_save;

	public static FormPrompt inst_prompt;
	public static JFrame form_prompt;
	public static FormEdit inst_edit;
	public static JFrame form_edit;

	public static File getSavePath() {
		return path_save;
	}

	public static void setSavePath(File path_save) {
		PhotoLayout.path_save = path_save;
	}

	public static void main(String[] args) {
		FormPrompt.main(args);
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

	public static void beginParse() {
		if (path_save==null) {
			JPanel gui = new JPanel(new BorderLayout(3, 3));
			JLabel label = new JLabel();
			label.setText("You did not set a save path!");
			gui.add(label);
			JOptionPane.showMessageDialog(null,gui);
		} else {
			form_prompt.setVisible(false);
			FormEdit.main(new String[]{});
		}
	}
}
