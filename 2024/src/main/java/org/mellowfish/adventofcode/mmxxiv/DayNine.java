package org.mellowfish.adventofcode.mmxxiv;

import java.util.ArrayList;

public class DayNine {
    private final FileSystem fileSystem;

    class FileSystem {
        private final ArrayList<Integer> blocks;
        private int fileCount;

        FileSystem() {
            this.blocks = new ArrayList<>();
            this.fileCount = 0;
        }

        public void writeNewFile(int start, int length) {
//            if (length == 0) {
//                return;
//            }
            for (int i = 0; i < length; i++) {
                int index = start + i;
                if (blocks.size() == index) {
                    blocks.add(fileCount);
                } else if (blocks.size() < index) {
                    throw new RuntimeException(
                            String.format("Ran off end of file system! (%d < %d)", blocks.size(), index));
                } else {
                    blocks.set(index, fileCount);
                }
            }
            fileCount += 1;
        }

        public void clearSpace(int start, int length) {
            for (int i = 0; i < length; i++) {
                int index = start + i;
                if (blocks.size() == index) {
                    blocks.add(null);
                } else if (blocks.size() < index) {
                    throw new RuntimeException(
                            String.format("Ran off end of file system! (%d < %d)", blocks.size(), index));
                } else {
                    blocks.set(index, null);
                }
            }
        }

        public int checksum() {
            int total = 0;
            for (int i = 0; i < blocks.size(); i++) {
                Integer fileId = blocks.get(i);
                if (fileId == null) {
                    return total;
                }
                total += i * fileId;
            }
            return total;
        }

        public void compact() {
            int freeSpace = findNextFreeSpace(-1);
            int fileBlock = findPreviousFileBlock(blocks.size());
            while (freeSpace < fileBlock) {
                blocks.set(freeSpace, blocks.get(fileBlock));
                blocks.set(fileBlock, null);
                freeSpace = findNextFreeSpace(freeSpace);
                fileBlock = findPreviousFileBlock(fileBlock);
            }
        }

        private int findNextFreeSpace(int index) {
            do {
                index += 1;
            } while (blocks.get(index) != null);

            return index;
        }

        private int findPreviousFileBlock(int index) {
            do {
                index -= 1;
            } while (blocks.get(index) == null);

            return index;
        }

        public void print() {
            int fileBlocks = 0;
            int clearBlocks = 0;
            int rows = (blocks.size() / 20) + 1;
            int maxRowWidth = String.valueOf(rows).length();
            for(int i = 0; i < blocks.size(); i++) {
                if (i % 20 == 0) {
                    System.out.println();
                    System.out.printf("%" + maxRowWidth + "d: ", (i / 20) + 1);
                }
                Integer fileId = blocks.get(i);
                if (fileId == null) {
                    clearBlocks++;
                    System.out.print("_".repeat(fileIdWidth()) + " ");
                } else {
                    fileBlocks++;
                    System.out.printf("%" + fileIdWidth() + "s ", fileId);
                }
            }
            System.out.printf("%n%d file blocks, %d clear blocks%n", fileBlocks, clearBlocks);
        }

        int fileIdWidth() {
            return String.valueOf(fileCount - 1).length();
        }
    }

    public static DayNine with(String input) {
        return new DayNine(input);
    }

    DayNine(String input) {
        this.fileSystem = new FileSystem();
        parseInput(input);
    }

    public int partOne() {
        fileSystem.print();
        fileSystem.compact();
        fileSystem.print();
        return fileSystem.checksum();
    }

    public int partTwo() {
        return 42;
    }

    void parseInput(String input) {
        int block = 0;
        boolean isFile = true;
        for (int i = 0; i < input.length(); i++) {
            int blockLength = input.charAt(i) - '0';
            if (isFile) {
                fileSystem.writeNewFile(block, blockLength);
            } else {
                fileSystem.clearSpace(block, blockLength);
            }
            block += blockLength;
            isFile = !isFile;
        }
    }
}
