package imgs;

import java.awt.BorderLayout;
import java.awt.Color;
import java.awt.Dimension;
import java.awt.Font;
import java.awt.Graphics;
import java.awt.image.BufferedImage;
import java.util.Stack;
import javax.swing.JButton;
import javax.swing.JFrame;
import javax.swing.JPanel;
import javax.swing.UIManager;
import javax.swing.UnsupportedLookAndFeelException;

public class ImageUI {

  String[] btnStrs = new String[]{
      "选择文件", "保存", "原图", "灰度", "轮廓检测", "马赛克",
      "油画", "融合", "高斯模糊", "锐化", "直方图均衡",
      "撤回", "清空"
  };
  ImageListener imagel = new ImageListener();

  public static void main(String[] args)
      throws UnsupportedLookAndFeelException, ClassNotFoundException, InstantiationException, IllegalAccessException {
    new ImageUI().initUI();
  }

  public void initUI()
      throws UnsupportedLookAndFeelException, ClassNotFoundException, InstantiationException, IllegalAccessException {
    JFrame mainFrame = new JFrame("GraphToy");
    mainFrame.setSize(1600, 900);
    mainFrame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);

    BtnPanel btnPanel = new BtnPanel();
    btnPanel.setBackground(new Color(100, 100, 100));
    btnPanel.setPreferredSize(new Dimension(180, 0));

    DrawPanel drawPanel = new DrawPanel();
    drawPanel.setBackground(new Color(80, 80, 80));
    drawPanel.setImageStack(imagel.getImageStack());
    drawPanel.setPreferredSize(new Dimension(mainFrame.getWidth() - btnPanel.getWidth(), 0));
    //drawPanel.addMouseListener(imagel);
    //drawPanel.addMouseMotionListener(imagel);

    Font btnFont = new Font("微软雅黑", 0, 18);
    Dimension btnDim = new Dimension(150, 50);
    for (String btnStr : btnStrs) {
      JButton btn = new JButton(btnStr);
      btn.setBackground(Color.WHITE);
      btn.setFont(btnFont);
      btn.setPreferredSize(btnDim);
      btn.addActionListener(imagel);
      btnPanel.add(btn);
    }

    mainFrame.add(drawPanel, BorderLayout.CENTER);
    mainFrame.add(btnPanel, BorderLayout.EAST);
    UIManager.setLookAndFeel(UIManager.getSystemLookAndFeelClassName());
    mainFrame.setVisible(true);

    Graphics g = drawPanel.getGraphics();
    imagel.setGraphics(g);
    imagel.setDrawPanel(drawPanel);
  }

}

class DrawPanel extends JPanel {

  Stack<BufferedImage> buffImgStack = null;

  public void setImageStack(Stack<BufferedImage> buffImgStack) {
    this.buffImgStack = buffImgStack;
  }

  @Override
  public void paint(Graphics g) {
    super.paint(g);
    if (buffImgStack == null) {
      return;
    }
    if (buffImgStack.empty()) {
      return;
    }
    BufferedImage buffImg = buffImgStack.peek();

    float w, h;
    float imgWidth = buffImg.getWidth();
    float imgHeight = buffImg.getHeight();
    float widthScale = this.getWidth() / imgWidth;
    float heightScale = this.getHeight() / imgHeight;
    if (widthScale < 1.0f) {
      if (heightScale < widthScale) {
        h = Math.min(imgHeight, this.getHeight());
        w = h * (imgWidth / imgHeight);
      } else {
        w = Math.min(imgWidth, this.getWidth());
        h = w * (imgHeight / imgWidth);
      }
    } else {
      h = Math.min(imgHeight, this.getHeight());
      w = h * (imgWidth / imgHeight);
    }
    g.drawImage(buffImg, 0, 0, (int) w, (int) h, null);
  }

}

class BtnPanel extends JPanel {

  @Override
  public void paint(Graphics g) {
    super.paint(g);
  }
}
