import javax.swing.*;
import java.awt.event.MouseAdapter;
import java.awt.event.MouseEvent;
import java.io.File;
import java.util.HashSet;

public class FormEdit {
	private JButton YEETButton;
	private JPanel imgpanel;
	private JPanel panel;
	private static JFrame frame;
	private HashSet<File> selectedFiles;
	public FormEdit(HashSet<File>) {
		YEETButton.addMouseListener(new MouseAdapter() {
			@Override
			public void mouseClicked(MouseEvent e) {
				frame.dispose();
			}
		});
	}

	public static void init(HashSet images) {
		main(new String[]{});
		selectedFiles = images;
	}

	public static void main(String[] args) {
		frame = new JFrame("FormEdit");
		frame.setContentPane(new FormEdit(selectedFiles).panel);
		frame.setDefaultCloseOperation(JFrame.DISPOSE_ON_CLOSE);
		frame.pack();
		frame.setVisible(true);
	}
}
