PROJECT_NAME = graphics

SRC_DIR = src
INC_DIR = include
OBJ_DIR = obj
BUILD_DIR = build

CC = gcc
CFLAGS = -Wall -Werror -Wextra -pedantic --std=c17 -g -O3 -I$(INC_DIR) -lSDL2 -lSDL2main

SRC = $(shell find $(SRC_DIR) -name '*.c')
OBJ = $(patsubst $(SRC_DIR)/%.c, $(OBJ_DIR)/%.o, $(SRC))

TARGET = $(BUILD_DIR)/$(PROJECT_NAME).out

all: $(TARGET)

$(TARGET): $(OBJ)
	@mkdir -p $(BUILD_DIR)
	$(CC) $(CFLAGS) -o $(TARGET) $(OBJ)

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.c
	@mkdir -p $(OBJ_DIR)
	$(CC) $(CFLAGS) -MMD -MP -c $< -o $@

clean:
	rm -rf $(OBJ_DIR) $(BUILD_DIR)

.PHONY: all clean 
