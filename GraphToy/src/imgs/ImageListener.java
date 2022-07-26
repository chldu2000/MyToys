package imgs;

import java.awt.Graphics;
import java.awt.event.ActionEvent;
import java.awt.event.MouseEvent;
import java.awt.image.BufferedImage;
import java.io.File;
import java.io.IOException;
import java.util.Stack;
import javax.imageio.ImageIO;
import javax.swing.JButton;
import javax.swing.JFileChooser;
import javax.swing.filechooser.FileFilter;

public class ImageListener extends ListenerImg {

  Graphics gr;
  ImageUtils imageUtils;
  int[][] imgArr;
  // 当前对象
  //BufferedImage buffImg;
  Stack<BufferedImage> buffImgStack = new Stack<>();
  DrawPanel drawPanel;
  /**
   * 只保存 PNG 文件
   */
  FileFilter pngFilter = new FileFilter() {
    @Override
    public boolean accept(File f) {
      return f.getName().endsWith(".png");
    }

    @Override
    public String getDescription() {
      return "png 文件";
    }
  };

  public void setGraphics(Graphics gr) {
    this.gr = gr;
    initImageUtils(gr);
  }

  public Stack<BufferedImage> getImageStack() {
    return this.buffImgStack;
  }

  public void setDrawPanel(DrawPanel drawPanel) {
    this.drawPanel = drawPanel;
  }

  public void initImageUtils(Graphics gr) {
    imageUtils = new ImageUtils(gr);
    // imgArr = imageUtils.imageFileToArr("D:\\Learning\\extra\\exGraph\\src\\imgs\\hnu.jpg");
  }

  @Override
  public void actionPerformed(ActionEvent e) {
    JButton btn = (JButton) e.getSource();
    String btnStr = btn.getActionCommand();
    System.out.println("Selected: " + btnStr);
    switch (btnStr) {
      case "保存": {
        JFileChooser fileChooser = new JFileChooser(System.getProperty("user.home"));
        fileChooser.setFileFilter(pngFilter);
        if (fileChooser.showSaveDialog(null) == JFileChooser.APPROVE_OPTION) {
          try {
            File target = fileChooser.getSelectedFile();
            // 补充后缀
            if (!target.getPath().endsWith(".png")) {
              target = new File(target.getPath() + ".png");
            }
            System.out.println("Save: " + ImageIO.write(buffImgStack.peek(),
                "png", target));
            System.out.println(target.getPath());
          } catch (IOException ex) {
            ex.printStackTrace();
          }
        }
        break;
      }
      case "选择文件": {
        JFileChooser fileChooser = new JFileChooser(System.getProperty("user.home"));
        if (fileChooser.showOpenDialog(null) == JFileChooser.APPROVE_OPTION) {
          imgArr = imageUtils.imageFileToArr(fileChooser.getSelectedFile().getPath());
        }
      }
      case "原图": {
        imageUtils.drawImage(imgArr);
        buffImgStack.push(imageUtils.getBuffImg());
        break;
      }
      case "灰度": {
        imageUtils.drawGrayImage(imgArr);
        buffImgStack.push(imageUtils.getBuffImg());
        break;
      }
      case "轮廓检测": {
        imageUtils.drawContour(imgArr);
        buffImgStack.push(imageUtils.getBuffImg());
        break;
      }
      case "马赛克": {
        imageUtils.drawMosaic(imgArr);
        buffImgStack.push(imageUtils.getBuffImg());
        break;
      }
      case "油画": {
        imageUtils.drawOilPainting(imgArr);
        buffImgStack.push(imageUtils.getBuffImg());
        break;
      }
      case "融合": {
        imageUtils.drawFusion(imgArr);
        buffImgStack.push(imageUtils.getBuffImg());
        break;
      }
      case "高斯模糊": {
        imageUtils.drawGaussianBlur(imgArr);
        buffImgStack.push(imageUtils.getBuffImg());
        break;
      }
      case "锐化": {
        imageUtils.drawSharpenImage(imgArr);
        buffImgStack.push(imageUtils.getBuffImg());
        break;
      }
      case "直方图均衡": {
        imageUtils.histogramEqualization(imgArr);
        buffImgStack.push(imageUtils.getBuffImg());
        break;
      }
      case "撤回": {
        if (!buffImgStack.empty()) {
          buffImgStack.pop();
        }
        break;
      }
      case "清空": {
        buffImgStack.removeAllElements();
        break;
      }
      default:
        break;
    }
//        buffImg = imageUtils.getBuffImg();
//        buffImgList.add(buffImg);
    drawPanel.paint(gr);
  }

  @Override
  public void mouseClicked(MouseEvent e) {

  }

  @Override
  public void mousePressed(MouseEvent e) {

  }


}
