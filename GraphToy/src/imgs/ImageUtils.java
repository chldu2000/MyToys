package imgs;

import java.awt.Color;
import java.awt.Graphics;
import java.awt.image.BufferedImage;
import java.io.File;
import java.io.IOException;
import java.util.Random;
import javax.imageio.ImageIO;
import javax.swing.JFileChooser;

public class ImageUtils {

  public Graphics gr;
  BufferedImage buffImg;
  private int[][] anoSharpenCore3x3 = {{-1, 0, 1}, {-1, 0, 1}, {-1, 0, 1}};
  // private int[][] disVertical3x3 = {{0, -2, 0}, {0, 5, 0}, {0, -2, 0}};

  public ImageUtils(Graphics gr) {
    this.gr = gr;
  }

  public BufferedImage getBuffImg() {
    return this.buffImg;
  }

  // 将图片像素点的RGB值存入数组
  public int[][] imageFileToArr(String path) {
    File imgFile = new File(path);
    BufferedImage buffImg = null;
    try {
      buffImg = ImageIO.read(imgFile);
    } catch (IOException e) {
      e.printStackTrace();
    }
    int w = buffImg.getWidth();
    int h = buffImg.getHeight();
    int[][] imgArr = new int[w][h];
    for (int i = 0; i < w; i++) {
      for (int j = 0; j < h; j++) {
        imgArr[i][j] = buffImg.getRGB(i, j);
      }
    }
    return imgArr;
  }

  public void drawImage(int[][] imgArr) {
    buffImg = new BufferedImage(imgArr.length, imgArr[0].length,
        BufferedImage.TYPE_INT_ARGB);
    for (int i = 0; i < imgArr.length; i++) {
      for (int j = 0; j < imgArr[i].length; j++) {
        int rgbValue = imgArr[i][j];
        buffImg.setRGB(i, j, rgbValue);
      }
    }
    gr.drawImage(buffImg, 0, 0, null);
  }

  public void drawGrayImage(int[][] imgArr) {
    buffImg = new BufferedImage(imgArr.length, imgArr[0].length,
        BufferedImage.TYPE_BYTE_GRAY);
    Graphics bfg = buffImg.getGraphics();
    for (int i = 0; i < imgArr.length; i++) {
      for (int j = 0; j < imgArr[i].length; j++) {
        int rgbValue = imgArr[i][j];
        //buffImg.setRGB(i, j, rgbValue);
        int red = (rgbValue >> 16) & 0XFF;
        int green = (rgbValue >> 8) & 0XFF;
        int blue = rgbValue & 0XFF;
        int gray = (red + green + blue) / 3;
        bfg.setColor(new Color(gray, gray, gray));
        bfg.fillRect(i, j, 1, 1);
      }
    }
    gr.drawImage(buffImg, 0, 0, null);
  }

  public void drawContour(int[][] imgArr) {
    buffImg = new BufferedImage(imgArr.length, imgArr[0].length,
        BufferedImage.TYPE_INT_ARGB);
    for (int i = 0; i < imgArr.length - 3; i++) {
      for (int j = 0; j < imgArr[i].length - 3; j++) {
        int rgbValue = imgArr[i][j];
        int red = (rgbValue >> 16) & 0XFF;
        int green = (rgbValue >> 8) & 0XFF;
        int blue = rgbValue & 0XFF;
        int gray = (red + green + blue) / 3;

        int rgbValueNext = imgArr[i + 3][j + 3];
        int redNext = (rgbValueNext >> 16) & 0XFF;
        int greenNext = (rgbValueNext >> 8) & 0XFF;
        int blueNext = rgbValueNext & 0XFF;
        int grayNext = (redNext + greenNext + blueNext) / 3;

        if (Math.abs(gray - grayNext) > 16) {
          buffImg.setRGB(i, j, rgbValue);
        } else {
          buffImg.setRGB(i, j, 0xFFFFFFFF);
        }
      }
    }
    gr.drawImage(buffImg, 0, 0, null);
  }

  public void drawMosaic(int[][] imgArr) {
    buffImg = new BufferedImage(imgArr.length, imgArr[0].length,
        BufferedImage.TYPE_INT_ARGB);
    Graphics bfg = buffImg.getGraphics();
    int w = imgArr.length >= 256 ? (imgArr.length / 128) : 2;
    for (int i = 0; i < imgArr.length; i += w) {
      for (int j = 0; j < imgArr[i].length; j += w) {
        int rgbValue = imgArr[i][j];
        Color color = new Color(rgbValue);
        // bfg 通用
        bfg.setColor(color);
        bfg.fillRect(i, j, w, w);
      }
    }
    gr.drawImage(buffImg, 0, 0, null);
  }

  public void drawOilPainting(int[][] imgArr) {
    buffImg = new BufferedImage(imgArr.length, imgArr[0].length,
        BufferedImage.TYPE_INT_ARGB);
    Graphics bfg = buffImg.getGraphics();

    for (int i = 0; i < imgArr.length; i += 4) {
      for (int j = 0; j < imgArr[i].length; j += 5) {
        int rgbValue = imgArr[i][j];
        Color color = new Color(rgbValue);
        Random random = new Random();
        int w = random.nextInt(18) + 4;
        int h = random.nextInt(22) + 5;
        bfg.setColor(color);
        bfg.fillOval(i, j, w, h);
      }
    }
    gr.drawImage(buffImg, 0, 0, null);
  }

  public void drawFusion(int[][] imgArr) {
    // 默认打开用户主目录
    JFileChooser fileChooser = new JFileChooser(
            System.getProperty("user.home"));
    fileChooser.showOpenDialog(null);
    int[][] imgArr2 = imageFileToArr(fileChooser.getSelectedFile().getPath());
    int maxWidth = Math.max(imgArr.length, imgArr2.length);
    int maxHeight = Math.max(imgArr[0].length, imgArr2[0].length);
    buffImg = new BufferedImage(maxWidth, maxHeight,
        BufferedImage.TYPE_INT_ARGB);
    Graphics bfg = buffImg.getGraphics();
    for (int i = 0; i < maxWidth; i++) {
      for (int j = 0; j < maxHeight; j++) {
        int rgbValue1 = (i < imgArr.length && j < imgArr[0].length) ? imgArr[i][j] : 0;
        int rgbValue2 = (i < imgArr2.length && j < imgArr2[0].length) ? imgArr2[i][j] : 0;

        int red1 = (rgbValue1 >> 16) & 0XFF;
        int green1 = (rgbValue1 >> 8) & 0XFF;
        int blue1 = rgbValue1 & 0XFF;
        int red2 = (rgbValue2 >> 16) & 0XFF;
        int green2 = (rgbValue2 >> 8) & 0XFF;
        int blue2 = rgbValue2 & 0XFF;

        int red = (red1 + red2) / 2;
        int green = (green1 + green2) / 2;
        int blue = (blue1 + blue2) / 2;
        // int rgbValue = (red * 65536) + (green *256) + blue;
        // buffImg.setRGB(i, j, rgbValue);
        bfg.setColor(new Color(red, green, blue));
        bfg.fillRect(i, j, 1, 1);
      }
    }
    gr.drawImage(buffImg, 0, 0, null);
  }

  // 获取以点 (i,j) 为中心的九个像素点的 RGB 值
  private int[][] rgbValueMatrix3x3(int[][] imgArr, int i, int j) {
    return new int[][]{{imgArr[i - 1][j - 1], imgArr[i][j - 1], imgArr[i + 1][j - 1]},
        {imgArr[i - 1][j], imgArr[i][j], imgArr[i + 1][j]},
        {imgArr[i - 1][j + 1], imgArr[i][j + 1], imgArr[i + 1][j + 1]}};
  }

  private int gaussianBlur3x3(int[][] values) {
    return (values[0][0] * 4 + values[0][1] * 7 + values[0][2] * 4 +
        values[1][0] * 7 + values[1][1] * 20 + values[1][2] * 7 +
        values[2][0] * 4 + values[2][1] * 7 + values[2][2] * 4) / 64;
  }

  public void drawGaussianBlur(int[][] imgArr) {
    int width = imgArr.length;
    int height = imgArr[0].length;
    buffImg = new BufferedImage(width, height,
        BufferedImage.TYPE_INT_ARGB);
    Graphics bfg = buffImg.getGraphics();
    for (int i = 0; i < width; i++) {
      for (int j = 0; j < height; j++) {
        if (i == 0 || j == 0 || i == width - 1 || j == height - 1) {
          buffImg.setRGB(i, j, imgArr[i][j]);
        } else {
          int[][] rgbValues = rgbValueMatrix3x3(imgArr, i, j);
          int[][] redValues = new int[3][3];
          int[][] greenValues = new int[3][3];
          int[][] blueValues = new int[3][3];
          for (int k = 0; k < 3; k++) {
            for (int l = 0; l < 3; l++) {
              redValues[k][l] = (rgbValues[k][l] >> 16) & 0xFF;
              greenValues[k][l] = (rgbValues[k][l] >> 8) & 0xFF;
              blueValues[k][l] = rgbValues[k][l] & 0xFF;
            }
          }
          int red = gaussianBlur3x3(redValues);
          int green = gaussianBlur3x3(greenValues);
          int blue = gaussianBlur3x3(blueValues);
          bfg.setColor(new Color(red, green, blue));
          bfg.fillRect(i, j, 1, 1);
        }
      }
    }
    gr.drawImage(buffImg, 0, 0, null);
  }

  private int sharpen3x3(int[][] values) {
    return values[0][0] * (-1) + values[0][1] * (-1) + values[0][2] * (-1) +
        values[1][0] * (-1) + values[1][1] * 9 + values[1][2] * (-1) +
        values[2][0] * (-1) + values[2][1] * (-1) + values[2][2] * (-1);
  }

  public void drawSharpenImage(int[][] imgArr) {
    int width = imgArr.length;
    int height = imgArr[0].length;
    buffImg = new BufferedImage(width, height,
        BufferedImage.TYPE_INT_ARGB);
    Graphics bfg = buffImg.getGraphics();
    for (int i = 0; i < width; i++) {
      for (int j = 0; j < height; j++) {
        if (i == 0 || j == 0 || i == width - 1 || j == height - 1) {
          buffImg.setRGB(i, j, imgArr[i][j]);
        } else {
          int[][] rgbValues = rgbValueMatrix3x3(imgArr, i, j);
          int[][] redValues = new int[3][3];
          int[][] greenValues = new int[3][3];
          int[][] blueValues = new int[3][3];
          for (int k = 0; k < 3; k++) {
            for (int l = 0; l < 3; l++) {
              redValues[k][l] = (rgbValues[k][l] >> 16) & 0xFF;
              greenValues[k][l] = (rgbValues[k][l] >> 8) & 0xFF;
              blueValues[k][l] = rgbValues[k][l] & 0xFF;
            }
          }
          int red = sharpen3x3(redValues);
          int green = sharpen3x3(greenValues);
          int blue = sharpen3x3(blueValues);
          red = Math.max(0, red);
          green = Math.max(0, green);
          blue = Math.max(0, blue);
          red = Math.min(255, red);
          green = Math.min(255, green);
          blue = Math.min(255, blue);
          bfg.setColor(new Color(red, green, blue));
          bfg.fillRect(i, j, 1, 1);
        }
      }
    }
    gr.drawImage(buffImg, 0, 0, null);
  }

  // 由 RGB 值算出灰度值
  private int rgbToGray(int rgbValue) {
    int red = (rgbValue >> 16) & 0XFF;
    int green = (rgbValue >> 8) & 0XFF;
    int blue = rgbValue & 0XFF;
    return (red + green + blue) / 3;
  }

  
  // 直方图均衡化
  public void histogramEqualization(int[][] imgArr) {
    int width = imgArr.length; // 图像宽度
    int height = imgArr[0].length; // 图像高度
    int[][] grayValue = new int[width][height]; // 每个点的原灰度值
    int[] count = new int[256]; // 每个灰度值对应的像素点数

    buffImg = new BufferedImage(width, height,
        BufferedImage.TYPE_BYTE_GRAY);
    Graphics bfg = buffImg.getGraphics();

    // 分256级，获取每个像素点的灰度值，计算每个灰度值的像素点个数
    for (int i = 0; i < width; i++) {
      for (int j = 0; j < height; j++) {
        int gray = rgbToGray(imgArr[i][j]);
        grayValue[i][j] = gray;
        count[gray] += 1;
      }
    }

    // 计算每个灰度级对应的新灰度级，即原来的灰度值均衡后的新灰度值
    int[] newGrayValue = new int[256];
    double sum = 0;
    int pixelNum = width * height;
    for (int i = 0; i < 256; i++) {
      sum += count[i];
      double scale = sum / pixelNum;
      newGrayValue[i] = (int) (scale * 255);
    }

    // 获取每个点对应的新灰度级，画出图像
    for (int i = 0; i < width; i++) {
      for (int j = 0; j < height; j++) {
        int currValue = newGrayValue[grayValue[i][j]];
        bfg.setColor(new Color(currValue, currValue, currValue));
        bfg.fillRect(i, j, 1, 1);
      }
    }
    gr.drawImage(buffImg, 0, 0, null);
  }

  // 指定卷积核进行处理
//  private int process3x3(int[][] values, int[][] core) {
//    int res = 0;
//    for (int i = 0; i < 3; i++) {
//      for (int j = 0; j < 3; j++) {
//        res += values[i][j] * core[i][j];
//      }
//    }
//    return res;
//  }

  // 防止 RGB 值越界
//  private int rgbLimit(int value) {
//    value = Math.max(0, value);
//    value = Math.min(255, value);
//    return value;
//  }
}
