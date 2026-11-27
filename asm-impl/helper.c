#include <inttypes.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>

typedef struct {
  uint16_t year;
  uint8_t month;
  uint8_t day;
} Date;

typedef struct {
  uint32_t price;
  Date date;
} Receipt;

void load_receipts_from_file(FILE *receipts_file, Receipt *receipts,
                             size_t size);
// extern void process(Receipt *receipts, uint32_t size);

int main() {
  size_t size = 100;

  Receipt receipts[size];

  FILE *receipts_file = fopen("receipts.txt", "r");

  load_receipts_from_file(receipts_file, receipts, size);

  // process(receipts, size);
}

void load_receipts_from_file(FILE *receipts_file, Receipt *receipts,
                             size_t size) {
  for (size_t i = 0; i < size; i++) {
    Receipt receipt;

    int result = fscanf(receipts_file,
        "%4" SCNu16 "-%2" SCNu8 "-%2" SCNu8 ",%" SCNu32,
        &receipt.date.year,
        &receipt.date.month,
        &receipt.date.day,
        &receipt.price
    );

    if (result != 4) {
        printf("Failed at receipt #%zu (matched %d fields)\n", i + 1, result);
        break;
    }

    printf("Receipt #%zu\t\t|\t\tDate: %04u-%02u-%02u\tPrice: %u\n",
           i + 1,
           receipt.date.year,
           receipt.date.month,
           receipt.date.day,
           receipt.price);
  }
}
