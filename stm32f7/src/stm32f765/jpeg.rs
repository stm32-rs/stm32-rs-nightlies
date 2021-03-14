#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - JPEG codec configuration register 0"]
    pub jpeg_confr0: JPEG_CONFR0,
    #[doc = "0x04 - JPEG codec configuration register 1"]
    pub jpeg_confr1: JPEG_CONFR1,
    #[doc = "0x08 - JPEG codec configuration register 2"]
    pub jpeg_confr2: JPEG_CONFR2,
    #[doc = "0x0c - JPEG codec configuration register 3"]
    pub jpeg_confr3: JPEG_CONFR3,
    #[doc = "0x10 - JPEG codec configuration register 4"]
    pub jpeg_confr4: JPEG_CONFR4,
    #[doc = "0x14 - JPEG codec configuration register 5"]
    pub jpeg_confr5: JPEG_CONFR5,
    #[doc = "0x18 - JPEG codec configuration register 6"]
    pub jpeg_confr6: JPEG_CONFR6,
    #[doc = "0x1c - JPEG codec configuration register 7"]
    pub jpeg_confr7: JPEG_CONFR7,
    _reserved8: [u8; 16usize],
    #[doc = "0x30 - JPEG control register"]
    pub jpeg_cr: JPEG_CR,
    #[doc = "0x34 - JPEG status register"]
    pub jpeg_sr: JPEG_SR,
    #[doc = "0x38 - JPEG clear flag register"]
    pub jpeg_cfr: JPEG_CFR,
    _reserved11: [u8; 4usize],
    #[doc = "0x40 - JPEG data input register"]
    pub jpeg_dir: JPEG_DIR,
    #[doc = "0x44 - JPEG data output register"]
    pub jpeg_dor: JPEG_DOR,
    _reserved13: [u8; 8usize],
    #[doc = "0x50 - JPEG quantization tables"]
    pub qmem0_0: QMEM0_0,
    #[doc = "0x54 - JPEG quantization tables"]
    pub qmem0_1: QMEM0_1,
    #[doc = "0x58 - JPEG quantization tables"]
    pub qmem0_2: QMEM0_2,
    #[doc = "0x5c - JPEG quantization tables"]
    pub qmem0_3: QMEM0_3,
    #[doc = "0x60 - JPEG quantization tables"]
    pub qmem0_4: QMEM0_4,
    #[doc = "0x64 - JPEG quantization tables"]
    pub qmem0_5: QMEM0_5,
    #[doc = "0x68 - JPEG quantization tables"]
    pub qmem0_6: QMEM0_6,
    #[doc = "0x6c - JPEG quantization tables"]
    pub qmem0_7: QMEM0_7,
    #[doc = "0x70 - JPEG quantization tables"]
    pub qmem0_8: QMEM0_8,
    #[doc = "0x74 - JPEG quantization tables"]
    pub qmem0_9: QMEM0_9,
    #[doc = "0x78 - JPEG quantization tables"]
    pub qmem0_10: QMEM0_10,
    #[doc = "0x7c - JPEG quantization tables"]
    pub qmem0_11: QMEM0_11,
    #[doc = "0x80 - JPEG quantization tables"]
    pub qmem0_12: QMEM0_12,
    #[doc = "0x84 - JPEG quantization tables"]
    pub qmem0_13: QMEM0_13,
    #[doc = "0x88 - JPEG quantization tables"]
    pub qmem0_14: QMEM0_14,
    #[doc = "0x8c - JPEG quantization tables"]
    pub qmem0_15: QMEM0_15,
    #[doc = "0x90 - JPEG quantization tables"]
    pub qmem1_0: QMEM1_0,
    #[doc = "0x94 - JPEG quantization tables"]
    pub qmem1_1: QMEM1_1,
    #[doc = "0x98 - JPEG quantization tables"]
    pub qmem1_2: QMEM1_2,
    #[doc = "0x9c - JPEG quantization tables"]
    pub qmem1_3: QMEM1_3,
    #[doc = "0xa0 - JPEG quantization tables"]
    pub qmem1_4: QMEM1_4,
    #[doc = "0xa4 - JPEG quantization tables"]
    pub qmem1_5: QMEM1_5,
    #[doc = "0xa8 - JPEG quantization tables"]
    pub qmem1_6: QMEM1_6,
    #[doc = "0xac - JPEG quantization tables"]
    pub qmem1_7: QMEM1_7,
    #[doc = "0xb0 - JPEG quantization tables"]
    pub qmem1_8: QMEM1_8,
    #[doc = "0xb4 - JPEG quantization tables"]
    pub qmem1_9: QMEM1_9,
    #[doc = "0xb8 - JPEG quantization tables"]
    pub qmem1_10: QMEM1_10,
    #[doc = "0xbc - JPEG quantization tables"]
    pub qmem1_11: QMEM1_11,
    #[doc = "0xc0 - JPEG quantization tables"]
    pub qmem1_12: QMEM1_12,
    #[doc = "0xc4 - JPEG quantization tables"]
    pub qmem1_13: QMEM1_13,
    #[doc = "0xc8 - JPEG quantization tables"]
    pub qmem1_14: QMEM1_14,
    #[doc = "0xcc - JPEG quantization tables"]
    pub qmem1_15: QMEM1_15,
    #[doc = "0xd0 - JPEG quantization tables"]
    pub qmem2_0: QMEM2_0,
    #[doc = "0xd4 - JPEG quantization tables"]
    pub qmem2_1: QMEM2_1,
    #[doc = "0xd8 - JPEG quantization tables"]
    pub qmem2_2: QMEM2_2,
    #[doc = "0xdc - JPEG quantization tables"]
    pub qmem2_3: QMEM2_3,
    #[doc = "0xe0 - JPEG quantization tables"]
    pub qmem2_4: QMEM2_4,
    #[doc = "0xe4 - JPEG quantization tables"]
    pub qmem2_5: QMEM2_5,
    #[doc = "0xe8 - JPEG quantization tables"]
    pub qmem2_6: QMEM2_6,
    #[doc = "0xec - JPEG quantization tables"]
    pub qmem2_7: QMEM2_7,
    #[doc = "0xf0 - JPEG quantization tables"]
    pub qmem2_8: QMEM2_8,
    #[doc = "0xf4 - JPEG quantization tables"]
    pub qmem2_9: QMEM2_9,
    #[doc = "0xf8 - JPEG quantization tables"]
    pub qmem2_10: QMEM2_10,
    #[doc = "0xfc - JPEG quantization tables"]
    pub qmem2_11: QMEM2_11,
    #[doc = "0x100 - JPEG quantization tables"]
    pub qmem2_12: QMEM2_12,
    #[doc = "0x104 - JPEG quantization tables"]
    pub qmem2_13: QMEM2_13,
    #[doc = "0x108 - JPEG quantization tables"]
    pub qmem2_14: QMEM2_14,
    #[doc = "0x10c - JPEG quantization tables"]
    pub qmem2_15: QMEM2_15,
    #[doc = "0x110 - JPEG quantization tables"]
    pub qmem3_0: QMEM3_0,
    #[doc = "0x114 - JPEG quantization tables"]
    pub qmem3_1: QMEM3_1,
    #[doc = "0x118 - JPEG quantization tables"]
    pub qmem3_2: QMEM3_2,
    #[doc = "0x11c - JPEG quantization tables"]
    pub qmem3_3: QMEM3_3,
    #[doc = "0x120 - JPEG quantization tables"]
    pub qmem3_4: QMEM3_4,
    #[doc = "0x124 - JPEG quantization tables"]
    pub qmem3_5: QMEM3_5,
    #[doc = "0x128 - JPEG quantization tables"]
    pub qmem3_6: QMEM3_6,
    #[doc = "0x12c - JPEG quantization tables"]
    pub qmem3_7: QMEM3_7,
    #[doc = "0x130 - JPEG quantization tables"]
    pub qmem3_8: QMEM3_8,
    #[doc = "0x134 - JPEG quantization tables"]
    pub qmem3_9: QMEM3_9,
    #[doc = "0x138 - JPEG quantization tables"]
    pub qmem3_10: QMEM3_10,
    #[doc = "0x13c - JPEG quantization tables"]
    pub qmem3_11: QMEM3_11,
    #[doc = "0x140 - JPEG quantization tables"]
    pub qmem3_12: QMEM3_12,
    #[doc = "0x144 - JPEG quantization tables"]
    pub qmem3_13: QMEM3_13,
    #[doc = "0x148 - JPEG quantization tables"]
    pub qmem3_14: QMEM3_14,
    #[doc = "0x14c - JPEG quantization tables"]
    pub qmem3_15: QMEM3_15,
    #[doc = "0x150 - JPEG HuffMin tables"]
    pub huffmin_0: HUFFMIN_0,
    #[doc = "0x154 - JPEG HuffMin tables"]
    pub huffmin_1: HUFFMIN_1,
    #[doc = "0x158 - JPEG HuffMin tables"]
    pub huffmin_2: HUFFMIN_2,
    #[doc = "0x15c - JPEG HuffMin tables"]
    pub huffmin_3: HUFFMIN_3,
    #[doc = "0x160 - JPEG HuffMin tables"]
    pub huffmin_4: HUFFMIN_4,
    #[doc = "0x164 - JPEG HuffMin tables"]
    pub huffmin_5: HUFFMIN_5,
    #[doc = "0x168 - JPEG HuffMin tables"]
    pub huffmin_6: HUFFMIN_6,
    #[doc = "0x16c - JPEG HuffMin tables"]
    pub huffmin_7: HUFFMIN_7,
    #[doc = "0x170 - JPEG HuffMin tables"]
    pub huffmin_8: HUFFMIN_8,
    #[doc = "0x174 - JPEG HuffMin tables"]
    pub huffmin_9: HUFFMIN_9,
    #[doc = "0x178 - JPEG HuffMin tables"]
    pub huffmin_10: HUFFMIN_10,
    #[doc = "0x17c - JPEG HuffMin tables"]
    pub huffmin_11: HUFFMIN_11,
    #[doc = "0x180 - JPEG HuffMin tables"]
    pub huffmin_12: HUFFMIN_12,
    #[doc = "0x184 - JPEG HuffMin tables"]
    pub huffmin_13: HUFFMIN_13,
    #[doc = "0x188 - JPEG HuffMin tables"]
    pub huffmin_14: HUFFMIN_14,
    #[doc = "0x18c - JPEG HuffMin tables"]
    pub huffmin_15: HUFFMIN_15,
    #[doc = "0x190 - JPEG HuffSymb tables"]
    pub huffbase0: HUFFBASE0,
    #[doc = "0x194 - JPEG HuffSymb tables"]
    pub huffbase1: HUFFBASE1,
    #[doc = "0x198 - JPEG HuffSymb tables"]
    pub huffbase2: HUFFBASE2,
    #[doc = "0x19c - JPEG HuffSymb tables"]
    pub huffbase3: HUFFBASE3,
    #[doc = "0x1a0 - JPEG HuffSymb tables"]
    pub huffbase4: HUFFBASE4,
    #[doc = "0x1a4 - JPEG HuffSymb tables"]
    pub huffbase5: HUFFBASE5,
    #[doc = "0x1a8 - JPEG HuffSymb tables"]
    pub huffbase6: HUFFBASE6,
    #[doc = "0x1ac - JPEG HuffSymb tables"]
    pub huffbase7: HUFFBASE7,
    #[doc = "0x1b0 - JPEG HuffSymb tables"]
    pub huffbase8: HUFFBASE8,
    #[doc = "0x1b4 - JPEG HuffSymb tables"]
    pub huffbase9: HUFFBASE9,
    #[doc = "0x1b8 - JPEG HuffSymb tables"]
    pub huffbase10: HUFFBASE10,
    #[doc = "0x1bc - JPEG HuffSymb tables"]
    pub huffbase11: HUFFBASE11,
    #[doc = "0x1c0 - JPEG HuffSymb tables"]
    pub huffbase12: HUFFBASE12,
    #[doc = "0x1c4 - JPEG HuffSymb tables"]
    pub huffbase13: HUFFBASE13,
    #[doc = "0x1c8 - JPEG HuffSymb tables"]
    pub huffbase14: HUFFBASE14,
    #[doc = "0x1cc - JPEG HuffSymb tables"]
    pub huffbase15: HUFFBASE15,
    #[doc = "0x1d0 - JPEG HuffSymb tables"]
    pub huffbase16: HUFFBASE16,
    #[doc = "0x1d4 - JPEG HuffSymb tables"]
    pub huffbase17: HUFFBASE17,
    #[doc = "0x1d8 - JPEG HuffSymb tables"]
    pub huffbase18: HUFFBASE18,
    #[doc = "0x1dc - JPEG HuffSymb tables"]
    pub huffbase19: HUFFBASE19,
    #[doc = "0x1e0 - JPEG HuffSymb tables"]
    pub huffbase20: HUFFBASE20,
    #[doc = "0x1e4 - JPEG HuffSymb tables"]
    pub huffbase21: HUFFBASE21,
    #[doc = "0x1e8 - JPEG HuffSymb tables"]
    pub huffbase22: HUFFBASE22,
    #[doc = "0x1ec - JPEG HuffSymb tables"]
    pub huffbase23: HUFFBASE23,
    #[doc = "0x1f0 - JPEG HuffSymb tables"]
    pub huffbase24: HUFFBASE24,
    #[doc = "0x1f4 - JPEG HuffSymb tables"]
    pub huffbase25: HUFFBASE25,
    #[doc = "0x1f8 - JPEG HuffSymb tables"]
    pub huffbase26: HUFFBASE26,
    #[doc = "0x1fc - JPEG HuffSymb tables"]
    pub huffbase27: HUFFBASE27,
    #[doc = "0x200 - JPEG HuffSymb tables"]
    pub huffbase28: HUFFBASE28,
    #[doc = "0x204 - JPEG HuffSymb tables"]
    pub huffbase29: HUFFBASE29,
    #[doc = "0x208 - JPEG HuffSymb tables"]
    pub huffbase30: HUFFBASE30,
    #[doc = "0x20c - JPEG HuffSymb tables"]
    pub huffbase31: HUFFBASE31,
    #[doc = "0x210 - JPEG HUFFSYMB tables"]
    pub huffsymb0: HUFFSYMB0,
    #[doc = "0x214 - JPEG HUFFSYMB tables"]
    pub huffsymb1: HUFFSYMB1,
    #[doc = "0x218 - JPEG HUFFSYMB tables"]
    pub huffsymb2: HUFFSYMB2,
    #[doc = "0x21c - JPEG HUFFSYMB tables"]
    pub huffsymb3: HUFFSYMB3,
    #[doc = "0x220 - JPEG HUFFSYMB tables"]
    pub huffsymb4: HUFFSYMB4,
    #[doc = "0x224 - JPEG HUFFSYMB tables"]
    pub huffsymb5: HUFFSYMB5,
    #[doc = "0x228 - JPEG HUFFSYMB tables"]
    pub huffsymb6: HUFFSYMB6,
    #[doc = "0x22c - JPEG HUFFSYMB tables"]
    pub huffsymb7: HUFFSYMB7,
    #[doc = "0x230 - JPEG HUFFSYMB tables"]
    pub huffsymb8: HUFFSYMB8,
    #[doc = "0x234 - JPEG HUFFSYMB tables"]
    pub huffsymb9: HUFFSYMB9,
    #[doc = "0x238 - JPEG HUFFSYMB tables"]
    pub huffsymb10: HUFFSYMB10,
    #[doc = "0x23c - JPEG HUFFSYMB tables"]
    pub huffsymb11: HUFFSYMB11,
    #[doc = "0x240 - JPEG HUFFSYMB tables"]
    pub huffsymb12: HUFFSYMB12,
    #[doc = "0x244 - JPEG HUFFSYMB tables"]
    pub huffsymb13: HUFFSYMB13,
    #[doc = "0x248 - JPEG HUFFSYMB tables"]
    pub huffsymb14: HUFFSYMB14,
    #[doc = "0x24c - JPEG HUFFSYMB tables"]
    pub huffsymb15: HUFFSYMB15,
    #[doc = "0x250 - JPEG HUFFSYMB tables"]
    pub huffsymb16: HUFFSYMB16,
    #[doc = "0x254 - JPEG HUFFSYMB tables"]
    pub huffsymb17: HUFFSYMB17,
    #[doc = "0x258 - JPEG HUFFSYMB tables"]
    pub huffsymb18: HUFFSYMB18,
    #[doc = "0x25c - JPEG HUFFSYMB tables"]
    pub huffsymb19: HUFFSYMB19,
    #[doc = "0x260 - JPEG HUFFSYMB tables"]
    pub huffsymb20: HUFFSYMB20,
    #[doc = "0x264 - JPEG HUFFSYMB tables"]
    pub huffsymb21: HUFFSYMB21,
    #[doc = "0x268 - JPEG HUFFSYMB tables"]
    pub huffsymb22: HUFFSYMB22,
    #[doc = "0x26c - JPEG HUFFSYMB tables"]
    pub huffsymb23: HUFFSYMB23,
    #[doc = "0x270 - JPEG HUFFSYMB tables"]
    pub huffsymb24: HUFFSYMB24,
    #[doc = "0x274 - JPEG HUFFSYMB tables"]
    pub huffsymb25: HUFFSYMB25,
    #[doc = "0x278 - JPEG HUFFSYMB tables"]
    pub huffsymb26: HUFFSYMB26,
    #[doc = "0x27c - JPEG HUFFSYMB tables"]
    pub huffsymb27: HUFFSYMB27,
    #[doc = "0x280 - JPEG HUFFSYMB tables"]
    pub huffsymb28: HUFFSYMB28,
    #[doc = "0x284 - JPEG HUFFSYMB tables"]
    pub huffsymb29: HUFFSYMB29,
    #[doc = "0x288 - JPEG HUFFSYMB tables"]
    pub huffsymb30: HUFFSYMB30,
    #[doc = "0x28c - JPEG HUFFSYMB tables"]
    pub huffsymb31: HUFFSYMB31,
    #[doc = "0x290 - JPEG HUFFSYMB tables"]
    pub huffsymb32: HUFFSYMB32,
    #[doc = "0x294 - JPEG HUFFSYMB tables"]
    pub huffsymb33: HUFFSYMB33,
    #[doc = "0x298 - JPEG HUFFSYMB tables"]
    pub huffsymb34: HUFFSYMB34,
    #[doc = "0x29c - JPEG HUFFSYMB tables"]
    pub huffsymb35: HUFFSYMB35,
    #[doc = "0x2a0 - JPEG HUFFSYMB tables"]
    pub huffsymb36: HUFFSYMB36,
    #[doc = "0x2a4 - JPEG HUFFSYMB tables"]
    pub huffsymb37: HUFFSYMB37,
    #[doc = "0x2a8 - JPEG HUFFSYMB tables"]
    pub huffsymb38: HUFFSYMB38,
    #[doc = "0x2ac - JPEG HUFFSYMB tables"]
    pub huffsymb39: HUFFSYMB39,
    #[doc = "0x2b0 - JPEG HUFFSYMB tables"]
    pub huffsymb40: HUFFSYMB40,
    #[doc = "0x2b4 - JPEG HUFFSYMB tables"]
    pub huffsymb41: HUFFSYMB41,
    #[doc = "0x2b8 - JPEG HUFFSYMB tables"]
    pub huffsymb42: HUFFSYMB42,
    #[doc = "0x2bc - JPEG HUFFSYMB tables"]
    pub huffsymb43: HUFFSYMB43,
    #[doc = "0x2c0 - JPEG HUFFSYMB tables"]
    pub huffsymb44: HUFFSYMB44,
    #[doc = "0x2c4 - JPEG HUFFSYMB tables"]
    pub huffsymb45: HUFFSYMB45,
    #[doc = "0x2c8 - JPEG HUFFSYMB tables"]
    pub huffsymb46: HUFFSYMB46,
    #[doc = "0x2cc - JPEG HUFFSYMB tables"]
    pub huffsymb47: HUFFSYMB47,
    #[doc = "0x2d0 - JPEG HUFFSYMB tables"]
    pub huffsymb48: HUFFSYMB48,
    #[doc = "0x2d4 - JPEG HUFFSYMB tables"]
    pub huffsymb49: HUFFSYMB49,
    #[doc = "0x2d8 - JPEG HUFFSYMB tables"]
    pub huffsymb50: HUFFSYMB50,
    #[doc = "0x2dc - JPEG HUFFSYMB tables"]
    pub huffsymb51: HUFFSYMB51,
    #[doc = "0x2e0 - JPEG HUFFSYMB tables"]
    pub huffsymb52: HUFFSYMB52,
    #[doc = "0x2e4 - JPEG HUFFSYMB tables"]
    pub huffsymb53: HUFFSYMB53,
    #[doc = "0x2e8 - JPEG HUFFSYMB tables"]
    pub huffsymb54: HUFFSYMB54,
    #[doc = "0x2ec - JPEG HUFFSYMB tables"]
    pub huffsymb55: HUFFSYMB55,
    #[doc = "0x2f0 - JPEG HUFFSYMB tables"]
    pub huffsymb56: HUFFSYMB56,
    #[doc = "0x2f4 - JPEG HUFFSYMB tables"]
    pub huffsymb57: HUFFSYMB57,
    #[doc = "0x2f8 - JPEG HUFFSYMB tables"]
    pub huffsymb58: HUFFSYMB58,
    #[doc = "0x2fc - JPEG HUFFSYMB tables"]
    pub huffsymb59: HUFFSYMB59,
    #[doc = "0x300 - JPEG HUFFSYMB tables"]
    pub huffsymb60: HUFFSYMB60,
    #[doc = "0x304 - JPEG HUFFSYMB tables"]
    pub huffsymb61: HUFFSYMB61,
    #[doc = "0x308 - JPEG HUFFSYMB tables"]
    pub huffsymb62: HUFFSYMB62,
    #[doc = "0x30c - JPEG HUFFSYMB tables"]
    pub huffsymb63: HUFFSYMB63,
    #[doc = "0x310 - JPEG HUFFSYMB tables"]
    pub huffsymb64: HUFFSYMB64,
    #[doc = "0x314 - JPEG HUFFSYMB tables"]
    pub huffsymb65: HUFFSYMB65,
    #[doc = "0x318 - JPEG HUFFSYMB tables"]
    pub huffsymb66: HUFFSYMB66,
    #[doc = "0x31c - JPEG HUFFSYMB tables"]
    pub huffsymb67: HUFFSYMB67,
    #[doc = "0x320 - JPEG HUFFSYMB tables"]
    pub huffsymb68: HUFFSYMB68,
    #[doc = "0x324 - JPEG HUFFSYMB tables"]
    pub huffsymb69: HUFFSYMB69,
    #[doc = "0x328 - JPEG HUFFSYMB tables"]
    pub huffsymb70: HUFFSYMB70,
    #[doc = "0x32c - JPEG HUFFSYMB tables"]
    pub huffsymb71: HUFFSYMB71,
    #[doc = "0x330 - JPEG HUFFSYMB tables"]
    pub huffsymb72: HUFFSYMB72,
    #[doc = "0x334 - JPEG HUFFSYMB tables"]
    pub huffsymb73: HUFFSYMB73,
    #[doc = "0x338 - JPEG HUFFSYMB tables"]
    pub huffsymb74: HUFFSYMB74,
    #[doc = "0x33c - JPEG HUFFSYMB tables"]
    pub huffsymb75: HUFFSYMB75,
    #[doc = "0x340 - JPEG HUFFSYMB tables"]
    pub huffsymb76: HUFFSYMB76,
    #[doc = "0x344 - JPEG HUFFSYMB tables"]
    pub huffsymb77: HUFFSYMB77,
    #[doc = "0x348 - JPEG HUFFSYMB tables"]
    pub huffsymb78: HUFFSYMB78,
    #[doc = "0x34c - JPEG HUFFSYMB tables"]
    pub huffsymb79: HUFFSYMB79,
    #[doc = "0x350 - JPEG HUFFSYMB tables"]
    pub huffsymb80: HUFFSYMB80,
    #[doc = "0x354 - JPEG HUFFSYMB tables"]
    pub huffsymb81: HUFFSYMB81,
    #[doc = "0x358 - JPEG HUFFSYMB tables"]
    pub huffsymb82: HUFFSYMB82,
    #[doc = "0x35c - JPEG HUFFSYMB tables"]
    pub huffsymb83: HUFFSYMB83,
    #[doc = "0x360 - JPEG DHTMem tables"]
    pub dhtmem0: DHTMEM0,
    #[doc = "0x364 - JPEG DHTMem tables"]
    pub dhtmem2: DHTMEM2,
    #[doc = "0x368 - JPEG DHTMem tables"]
    pub dhtmem3: DHTMEM3,
    #[doc = "0x36c - JPEG DHTMem tables"]
    pub dhtmem4: DHTMEM4,
    #[doc = "0x370 - JPEG DHTMem tables"]
    pub dhtmem5: DHTMEM5,
    #[doc = "0x374 - JPEG DHTMem tables"]
    pub dhtmem6: DHTMEM6,
    #[doc = "0x378 - JPEG DHTMem tables"]
    pub dhtmem7: DHTMEM7,
    #[doc = "0x37c - JPEG DHTMem tables"]
    pub dhtmem8: DHTMEM8,
    #[doc = "0x380 - JPEG DHTMem tables"]
    pub dhtmem9: DHTMEM9,
    #[doc = "0x384 - JPEG DHTMem tables"]
    pub dhtmem10: DHTMEM10,
    #[doc = "0x388 - JPEG DHTMem tables"]
    pub dhtmem11: DHTMEM11,
    #[doc = "0x38c - JPEG DHTMem tables"]
    pub dhtmem12: DHTMEM12,
    #[doc = "0x390 - JPEG DHTMem tables"]
    pub dhtmem13: DHTMEM13,
    #[doc = "0x394 - JPEG DHTMem tables"]
    pub dhtmem14: DHTMEM14,
    #[doc = "0x398 - JPEG DHTMem tables"]
    pub dhtmem15: DHTMEM15,
    #[doc = "0x39c - JPEG DHTMem tables"]
    pub dhtmem16: DHTMEM16,
    #[doc = "0x3a0 - JPEG DHTMem tables"]
    pub dhtmem17: DHTMEM17,
    #[doc = "0x3a4 - JPEG DHTMem tables"]
    pub dhtmem18: DHTMEM18,
    #[doc = "0x3a8 - JPEG DHTMem tables"]
    pub dhtmem19: DHTMEM19,
    #[doc = "0x3ac - JPEG DHTMem tables"]
    pub dhtmem20: DHTMEM20,
    #[doc = "0x3b0 - JPEG DHTMem tables"]
    pub dhtmem21: DHTMEM21,
    #[doc = "0x3b4 - JPEG DHTMem tables"]
    pub dhtmem22: DHTMEM22,
    #[doc = "0x3b8 - JPEG DHTMem tables"]
    pub dhtmem23: DHTMEM23,
    #[doc = "0x3bc - JPEG DHTMem tables"]
    pub dhtmem24: DHTMEM24,
    #[doc = "0x3c0 - JPEG DHTMem tables"]
    pub dhtmem25: DHTMEM25,
    #[doc = "0x3c4 - JPEG DHTMem tables"]
    pub dhtmem26: DHTMEM26,
    #[doc = "0x3c8 - JPEG DHTMem tables"]
    pub dhtmem27: DHTMEM27,
    #[doc = "0x3cc - JPEG DHTMem tables"]
    pub dhtmem28: DHTMEM28,
    #[doc = "0x3d0 - JPEG DHTMem tables"]
    pub dhtmem29: DHTMEM29,
    #[doc = "0x3d4 - JPEG DHTMem tables"]
    pub dhtmem30: DHTMEM30,
    #[doc = "0x3d8 - JPEG DHTMem tables"]
    pub dhtmem31: DHTMEM31,
    #[doc = "0x3dc - JPEG DHTMem tables"]
    pub dhtmem32: DHTMEM32,
    #[doc = "0x3e0 - JPEG DHTMem tables"]
    pub dhtmem33: DHTMEM33,
    #[doc = "0x3e4 - JPEG DHTMem tables"]
    pub dhtmem34: DHTMEM34,
    #[doc = "0x3e8 - JPEG DHTMem tables"]
    pub dhtmem35: DHTMEM35,
    #[doc = "0x3ec - JPEG DHTMem tables"]
    pub dhtmem36: DHTMEM36,
    #[doc = "0x3f0 - JPEG DHTMem tables"]
    pub dhtmem37: DHTMEM37,
    #[doc = "0x3f4 - JPEG DHTMem tables"]
    pub dhtmem38: DHTMEM38,
    #[doc = "0x3f8 - JPEG DHTMem tables"]
    pub dhtmem39: DHTMEM39,
    #[doc = "0x3fc - JPEG DHTMem tables"]
    pub dhtmem40: DHTMEM40,
    #[doc = "0x400 - JPEG DHTMem tables"]
    pub dhtmem41: DHTMEM41,
    #[doc = "0x404 - JPEG DHTMem tables"]
    pub dhtmem42: DHTMEM42,
    #[doc = "0x408 - JPEG DHTMem tables"]
    pub dhtmem43: DHTMEM43,
    #[doc = "0x40c - JPEG DHTMem tables"]
    pub dhtmem44: DHTMEM44,
    #[doc = "0x410 - JPEG DHTMem tables"]
    pub dhtmem45: DHTMEM45,
    #[doc = "0x414 - JPEG DHTMem tables"]
    pub dhtmem46: DHTMEM46,
    #[doc = "0x418 - JPEG DHTMem tables"]
    pub dhtmem47: DHTMEM47,
    #[doc = "0x41c - JPEG DHTMem tables"]
    pub dhtmem48: DHTMEM48,
    #[doc = "0x420 - JPEG DHTMem tables"]
    pub dhtmem49: DHTMEM49,
    #[doc = "0x424 - JPEG DHTMem tables"]
    pub dhtmem50: DHTMEM50,
    #[doc = "0x428 - JPEG DHTMem tables"]
    pub dhtmem51: DHTMEM51,
    #[doc = "0x42c - JPEG DHTMem tables"]
    pub dhtmem52: DHTMEM52,
    #[doc = "0x430 - JPEG DHTMem tables"]
    pub dhtmem53: DHTMEM53,
    #[doc = "0x434 - JPEG DHTMem tables"]
    pub dhtmem54: DHTMEM54,
    #[doc = "0x438 - JPEG DHTMem tables"]
    pub dhtmem55: DHTMEM55,
    #[doc = "0x43c - JPEG DHTMem tables"]
    pub dhtmem56: DHTMEM56,
    #[doc = "0x440 - JPEG DHTMem tables"]
    pub dhtmem57: DHTMEM57,
    #[doc = "0x444 - JPEG DHTMem tables"]
    pub dhtmem58: DHTMEM58,
    #[doc = "0x448 - JPEG DHTMem tables"]
    pub dhtmem59: DHTMEM59,
    #[doc = "0x44c - JPEG DHTMem tables"]
    pub dhtmem60: DHTMEM60,
    #[doc = "0x450 - JPEG DHTMem tables"]
    pub dhtmem61: DHTMEM61,
    #[doc = "0x454 - JPEG DHTMem tables"]
    pub dhtmem62: DHTMEM62,
    #[doc = "0x458 - JPEG DHTMem tables"]
    pub dhtmem63: DHTMEM63,
    #[doc = "0x45c - JPEG DHTMem tables"]
    pub dhtmem64: DHTMEM64,
    #[doc = "0x460 - JPEG DHTMem tables"]
    pub dhtmem65: DHTMEM65,
    #[doc = "0x464 - JPEG DHTMem tables"]
    pub dhtmem66: DHTMEM66,
    #[doc = "0x468 - JPEG DHTMem tables"]
    pub dhtmem67: DHTMEM67,
    #[doc = "0x46c - JPEG DHTMem tables"]
    pub dhtmem68: DHTMEM68,
    #[doc = "0x470 - JPEG DHTMem tables"]
    pub dhtmem69: DHTMEM69,
    #[doc = "0x474 - JPEG DHTMem tables"]
    pub dhtmem70: DHTMEM70,
    #[doc = "0x478 - JPEG DHTMem tables"]
    pub dhtmem71: DHTMEM71,
    #[doc = "0x47c - JPEG DHTMem tables"]
    pub dhtmem72: DHTMEM72,
    #[doc = "0x480 - JPEG DHTMem tables"]
    pub dhtmem73: DHTMEM73,
    #[doc = "0x484 - JPEG DHTMem tables"]
    pub dhtmem74: DHTMEM74,
    #[doc = "0x488 - JPEG DHTMem tables"]
    pub dhtmem75: DHTMEM75,
    #[doc = "0x48c - JPEG DHTMem tables"]
    pub dhtmem76: DHTMEM76,
    #[doc = "0x490 - JPEG DHTMem tables"]
    pub dhtmem77: DHTMEM77,
    #[doc = "0x494 - JPEG DHTMem tables"]
    pub dhtmem78: DHTMEM78,
    #[doc = "0x498 - JPEG DHTMem tables"]
    pub dhtmem79: DHTMEM79,
    #[doc = "0x49c - JPEG DHTMem tables"]
    pub dhtmem80: DHTMEM80,
    #[doc = "0x4a0 - JPEG DHTMem tables"]
    pub dhtmem81: DHTMEM81,
    #[doc = "0x4a4 - JPEG DHTMem tables"]
    pub dhtmem82: DHTMEM82,
    #[doc = "0x4a8 - JPEG DHTMem tables"]
    pub dhtmem83: DHTMEM83,
    #[doc = "0x4ac - JPEG DHTMem tables"]
    pub dhtmem84: DHTMEM84,
    #[doc = "0x4b0 - JPEG DHTMem tables"]
    pub dhtmem85: DHTMEM85,
    #[doc = "0x4b4 - JPEG DHTMem tables"]
    pub dhtmem86: DHTMEM86,
    #[doc = "0x4b8 - JPEG DHTMem tables"]
    pub dhtmem87: DHTMEM87,
    #[doc = "0x4bc - JPEG DHTMem tables"]
    pub dhtmem88: DHTMEM88,
    #[doc = "0x4c0 - JPEG DHTMem tables"]
    pub dhtmem89: DHTMEM89,
    #[doc = "0x4c4 - JPEG DHTMem tables"]
    pub dhtmem90: DHTMEM90,
    #[doc = "0x4c8 - JPEG DHTMem tables"]
    pub dhtmem91: DHTMEM91,
    #[doc = "0x4cc - JPEG DHTMem tables"]
    pub dhtmem92: DHTMEM92,
    #[doc = "0x4d0 - JPEG DHTMem tables"]
    pub dhtmem93: DHTMEM93,
    #[doc = "0x4d4 - JPEG DHTMem tables"]
    pub dhtmem94: DHTMEM94,
    #[doc = "0x4d8 - JPEG DHTMem tables"]
    pub dhtmem95: DHTMEM95,
    #[doc = "0x4dc - JPEG DHTMem tables"]
    pub dhtmem96: DHTMEM96,
    #[doc = "0x4e0 - JPEG DHTMem tables"]
    pub dhtmem97: DHTMEM97,
    #[doc = "0x4e4 - JPEG DHTMem tables"]
    pub dhtmem98: DHTMEM98,
    #[doc = "0x4e8 - JPEG DHTMem tables"]
    pub dhtmem99: DHTMEM99,
    #[doc = "0x4ec - JPEG DHTMem tables"]
    pub dhtmem100: DHTMEM100,
    #[doc = "0x4f0 - JPEG DHTMem tables"]
    pub dhtmem101: DHTMEM101,
    #[doc = "0x4f4 - JPEG DHTMem tables"]
    pub dhtmem102: DHTMEM102,
    #[doc = "0x4f8 - JPEG DHTMem tables"]
    pub dhtmem103: DHTMEM103,
    _reserved312: [u8; 4usize],
    #[doc = "0x500 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_0: HUFFENC_AC0_0,
    #[doc = "0x504 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_1: HUFFENC_AC0_1,
    #[doc = "0x508 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_2: HUFFENC_AC0_2,
    #[doc = "0x50c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_3: HUFFENC_AC0_3,
    #[doc = "0x510 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_4: HUFFENC_AC0_4,
    #[doc = "0x514 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_5: HUFFENC_AC0_5,
    #[doc = "0x518 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_6: HUFFENC_AC0_6,
    #[doc = "0x51c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_7: HUFFENC_AC0_7,
    #[doc = "0x520 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_8: HUFFENC_AC0_8,
    #[doc = "0x524 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_9: HUFFENC_AC0_9,
    #[doc = "0x528 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_10: HUFFENC_AC0_10,
    #[doc = "0x52c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_11: HUFFENC_AC0_11,
    #[doc = "0x530 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_12: HUFFENC_AC0_12,
    #[doc = "0x534 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_13: HUFFENC_AC0_13,
    #[doc = "0x538 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_14: HUFFENC_AC0_14,
    #[doc = "0x53c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_15: HUFFENC_AC0_15,
    #[doc = "0x540 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_16: HUFFENC_AC0_16,
    #[doc = "0x544 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_17: HUFFENC_AC0_17,
    #[doc = "0x548 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_18: HUFFENC_AC0_18,
    #[doc = "0x54c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_19: HUFFENC_AC0_19,
    #[doc = "0x550 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_20: HUFFENC_AC0_20,
    #[doc = "0x554 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_21: HUFFENC_AC0_21,
    #[doc = "0x558 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_22: HUFFENC_AC0_22,
    #[doc = "0x55c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_23: HUFFENC_AC0_23,
    #[doc = "0x560 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_24: HUFFENC_AC0_24,
    #[doc = "0x564 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_25: HUFFENC_AC0_25,
    #[doc = "0x568 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_26: HUFFENC_AC0_26,
    #[doc = "0x56c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_27: HUFFENC_AC0_27,
    #[doc = "0x570 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_28: HUFFENC_AC0_28,
    #[doc = "0x574 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_29: HUFFENC_AC0_29,
    #[doc = "0x578 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_30: HUFFENC_AC0_30,
    #[doc = "0x57c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_31: HUFFENC_AC0_31,
    #[doc = "0x580 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_32: HUFFENC_AC0_32,
    #[doc = "0x584 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_33: HUFFENC_AC0_33,
    #[doc = "0x588 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_34: HUFFENC_AC0_34,
    #[doc = "0x58c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_35: HUFFENC_AC0_35,
    #[doc = "0x590 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_36: HUFFENC_AC0_36,
    #[doc = "0x594 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_37: HUFFENC_AC0_37,
    #[doc = "0x598 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_38: HUFFENC_AC0_38,
    #[doc = "0x59c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_39: HUFFENC_AC0_39,
    #[doc = "0x5a0 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_40: HUFFENC_AC0_40,
    #[doc = "0x5a4 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_41: HUFFENC_AC0_41,
    #[doc = "0x5a8 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_42: HUFFENC_AC0_42,
    #[doc = "0x5ac - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_43: HUFFENC_AC0_43,
    #[doc = "0x5b0 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_44: HUFFENC_AC0_44,
    #[doc = "0x5b4 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_45: HUFFENC_AC0_45,
    #[doc = "0x5b8 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_46: HUFFENC_AC0_46,
    #[doc = "0x5bc - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_47: HUFFENC_AC0_47,
    #[doc = "0x5c0 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_48: HUFFENC_AC0_48,
    #[doc = "0x5c4 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_49: HUFFENC_AC0_49,
    #[doc = "0x5c8 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_50: HUFFENC_AC0_50,
    #[doc = "0x5cc - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_51: HUFFENC_AC0_51,
    #[doc = "0x5d0 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_52: HUFFENC_AC0_52,
    #[doc = "0x5d4 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_53: HUFFENC_AC0_53,
    #[doc = "0x5d8 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_54: HUFFENC_AC0_54,
    #[doc = "0x5dc - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_55: HUFFENC_AC0_55,
    #[doc = "0x5e0 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_56: HUFFENC_AC0_56,
    #[doc = "0x5e4 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_57: HUFFENC_AC0_57,
    #[doc = "0x5e8 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_58: HUFFENC_AC0_58,
    #[doc = "0x5ec - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_59: HUFFENC_AC0_59,
    #[doc = "0x5f0 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_60: HUFFENC_AC0_60,
    #[doc = "0x5f4 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_61: HUFFENC_AC0_61,
    #[doc = "0x5f8 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_62: HUFFENC_AC0_62,
    #[doc = "0x5fc - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_63: HUFFENC_AC0_63,
    #[doc = "0x600 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_64: HUFFENC_AC0_64,
    #[doc = "0x604 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_65: HUFFENC_AC0_65,
    #[doc = "0x608 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_66: HUFFENC_AC0_66,
    #[doc = "0x60c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_67: HUFFENC_AC0_67,
    #[doc = "0x610 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_68: HUFFENC_AC0_68,
    #[doc = "0x614 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_69: HUFFENC_AC0_69,
    #[doc = "0x618 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_70: HUFFENC_AC0_70,
    #[doc = "0x61c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_71: HUFFENC_AC0_71,
    #[doc = "0x620 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_72: HUFFENC_AC0_72,
    #[doc = "0x624 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_73: HUFFENC_AC0_73,
    #[doc = "0x628 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_74: HUFFENC_AC0_74,
    #[doc = "0x62c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_75: HUFFENC_AC0_75,
    #[doc = "0x630 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_76: HUFFENC_AC0_76,
    #[doc = "0x634 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_77: HUFFENC_AC0_77,
    #[doc = "0x638 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_78: HUFFENC_AC0_78,
    #[doc = "0x63c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_79: HUFFENC_AC0_79,
    #[doc = "0x640 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_80: HUFFENC_AC0_80,
    #[doc = "0x644 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_81: HUFFENC_AC0_81,
    #[doc = "0x648 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_82: HUFFENC_AC0_82,
    #[doc = "0x64c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_83: HUFFENC_AC0_83,
    #[doc = "0x650 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_84: HUFFENC_AC0_84,
    #[doc = "0x654 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_85: HUFFENC_AC0_85,
    #[doc = "0x658 - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_86: HUFFENC_AC0_86,
    #[doc = "0x65c - JPEG encoder, AC Huffman table 0"]
    pub huffenc_ac0_87: HUFFENC_AC0_87,
    #[doc = "0x660 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_0: HUFFENC_AC1_0,
    #[doc = "0x664 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_1: HUFFENC_AC1_1,
    #[doc = "0x668 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_2: HUFFENC_AC1_2,
    #[doc = "0x66c - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_3: HUFFENC_AC1_3,
    #[doc = "0x670 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_4: HUFFENC_AC1_4,
    #[doc = "0x674 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_5: HUFFENC_AC1_5,
    #[doc = "0x678 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_6: HUFFENC_AC1_6,
    #[doc = "0x67c - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_7: HUFFENC_AC1_7,
    #[doc = "0x680 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_8: HUFFENC_AC1_8,
    #[doc = "0x684 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_9: HUFFENC_AC1_9,
    #[doc = "0x688 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_10: HUFFENC_AC1_10,
    #[doc = "0x68c - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_11: HUFFENC_AC1_11,
    #[doc = "0x690 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_12: HUFFENC_AC1_12,
    #[doc = "0x694 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_13: HUFFENC_AC1_13,
    #[doc = "0x698 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_14: HUFFENC_AC1_14,
    #[doc = "0x69c - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_15: HUFFENC_AC1_15,
    #[doc = "0x6a0 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_16: HUFFENC_AC1_16,
    #[doc = "0x6a4 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_17: HUFFENC_AC1_17,
    #[doc = "0x6a8 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_18: HUFFENC_AC1_18,
    #[doc = "0x6ac - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_19: HUFFENC_AC1_19,
    #[doc = "0x6b0 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_20: HUFFENC_AC1_20,
    #[doc = "0x6b4 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_21: HUFFENC_AC1_21,
    #[doc = "0x6b8 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_22: HUFFENC_AC1_22,
    #[doc = "0x6bc - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_23: HUFFENC_AC1_23,
    #[doc = "0x6c0 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_24: HUFFENC_AC1_24,
    #[doc = "0x6c4 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_25: HUFFENC_AC1_25,
    #[doc = "0x6c8 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_26: HUFFENC_AC1_26,
    #[doc = "0x6cc - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_27: HUFFENC_AC1_27,
    #[doc = "0x6d0 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_28: HUFFENC_AC1_28,
    #[doc = "0x6d4 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_29: HUFFENC_AC1_29,
    #[doc = "0x6d8 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_30: HUFFENC_AC1_30,
    #[doc = "0x6dc - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_31: HUFFENC_AC1_31,
    #[doc = "0x6e0 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_32: HUFFENC_AC1_32,
    #[doc = "0x6e4 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_33: HUFFENC_AC1_33,
    #[doc = "0x6e8 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_34: HUFFENC_AC1_34,
    #[doc = "0x6ec - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_35: HUFFENC_AC1_35,
    #[doc = "0x6f0 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_36: HUFFENC_AC1_36,
    #[doc = "0x6f4 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_37: HUFFENC_AC1_37,
    #[doc = "0x6f8 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_38: HUFFENC_AC1_38,
    #[doc = "0x6fc - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_39: HUFFENC_AC1_39,
    #[doc = "0x700 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_40: HUFFENC_AC1_40,
    #[doc = "0x704 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_41: HUFFENC_AC1_41,
    #[doc = "0x708 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_42: HUFFENC_AC1_42,
    #[doc = "0x70c - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_43: HUFFENC_AC1_43,
    #[doc = "0x710 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_44: HUFFENC_AC1_44,
    #[doc = "0x714 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_45: HUFFENC_AC1_45,
    #[doc = "0x718 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_46: HUFFENC_AC1_46,
    #[doc = "0x71c - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_47: HUFFENC_AC1_47,
    #[doc = "0x720 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_48: HUFFENC_AC1_48,
    #[doc = "0x724 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_49: HUFFENC_AC1_49,
    #[doc = "0x728 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_50: HUFFENC_AC1_50,
    #[doc = "0x72c - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_51: HUFFENC_AC1_51,
    #[doc = "0x730 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_52: HUFFENC_AC1_52,
    #[doc = "0x734 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_53: HUFFENC_AC1_53,
    #[doc = "0x738 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_54: HUFFENC_AC1_54,
    #[doc = "0x73c - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_55: HUFFENC_AC1_55,
    #[doc = "0x740 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_56: HUFFENC_AC1_56,
    #[doc = "0x744 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_57: HUFFENC_AC1_57,
    #[doc = "0x748 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_58: HUFFENC_AC1_58,
    #[doc = "0x74c - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_59: HUFFENC_AC1_59,
    #[doc = "0x750 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_60: HUFFENC_AC1_60,
    #[doc = "0x754 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_61: HUFFENC_AC1_61,
    #[doc = "0x758 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_62: HUFFENC_AC1_62,
    #[doc = "0x75c - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_63: HUFFENC_AC1_63,
    #[doc = "0x760 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_64: HUFFENC_AC1_64,
    #[doc = "0x764 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_65: HUFFENC_AC1_65,
    #[doc = "0x768 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_66: HUFFENC_AC1_66,
    #[doc = "0x76c - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_67: HUFFENC_AC1_67,
    #[doc = "0x770 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_68: HUFFENC_AC1_68,
    #[doc = "0x774 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_69: HUFFENC_AC1_69,
    #[doc = "0x778 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_70: HUFFENC_AC1_70,
    #[doc = "0x77c - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_71: HUFFENC_AC1_71,
    #[doc = "0x780 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_72: HUFFENC_AC1_72,
    #[doc = "0x784 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_73: HUFFENC_AC1_73,
    #[doc = "0x788 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_74: HUFFENC_AC1_74,
    #[doc = "0x78c - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_75: HUFFENC_AC1_75,
    #[doc = "0x790 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_76: HUFFENC_AC1_76,
    #[doc = "0x794 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_77: HUFFENC_AC1_77,
    #[doc = "0x798 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_78: HUFFENC_AC1_78,
    #[doc = "0x79c - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_79: HUFFENC_AC1_79,
    #[doc = "0x7a0 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_80: HUFFENC_AC1_80,
    #[doc = "0x7a4 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_81: HUFFENC_AC1_81,
    #[doc = "0x7a8 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_82: HUFFENC_AC1_82,
    #[doc = "0x7ac - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_83: HUFFENC_AC1_83,
    #[doc = "0x7b0 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_84: HUFFENC_AC1_84,
    #[doc = "0x7b4 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_85: HUFFENC_AC1_85,
    #[doc = "0x7b8 - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_86: HUFFENC_AC1_86,
    #[doc = "0x7bc - JPEG encoder, AC Huffman table 1"]
    pub huffenc_ac1_87: HUFFENC_AC1_87,
    #[doc = "0x7c0 - JPEG encoder, DC Huffman table 0"]
    pub huffenc_dc0_0: HUFFENC_DC0_0,
    #[doc = "0x7c4 - JPEG encoder, DC Huffman table 0"]
    pub huffenc_dc0_1: HUFFENC_DC0_1,
    #[doc = "0x7c8 - JPEG encoder, DC Huffman table 0"]
    pub huffenc_dc0_2: HUFFENC_DC0_2,
    #[doc = "0x7cc - JPEG encoder, DC Huffman table 0"]
    pub huffenc_dc0_3: HUFFENC_DC0_3,
    #[doc = "0x7d0 - JPEG encoder, DC Huffman table 0"]
    pub huffenc_dc0_4: HUFFENC_DC0_4,
    #[doc = "0x7d4 - JPEG encoder, DC Huffman table 0"]
    pub huffenc_dc0_5: HUFFENC_DC0_5,
    #[doc = "0x7d8 - JPEG encoder, DC Huffman table 0"]
    pub huffenc_dc0_6: HUFFENC_DC0_6,
    #[doc = "0x7dc - JPEG encoder, DC Huffman table 0"]
    pub huffenc_dc0_7: HUFFENC_DC0_7,
    #[doc = "0x7e0 - JPEG encoder, DC Huffman table 1"]
    pub huffenc_dc1_0: HUFFENC_DC1_0,
    #[doc = "0x7e4 - JPEG encoder, DC Huffman table 1"]
    pub huffenc_dc1_1: HUFFENC_DC1_1,
    #[doc = "0x7e8 - JPEG encoder, DC Huffman table 1"]
    pub huffenc_dc1_2: HUFFENC_DC1_2,
    #[doc = "0x7ec - JPEG encoder, DC Huffman table 1"]
    pub huffenc_dc1_3: HUFFENC_DC1_3,
    #[doc = "0x7f0 - JPEG encoder, DC Huffman table 1"]
    pub huffenc_dc1_4: HUFFENC_DC1_4,
    #[doc = "0x7f4 - JPEG encoder, DC Huffman table 1"]
    pub huffenc_dc1_5: HUFFENC_DC1_5,
    #[doc = "0x7f8 - JPEG encoder, DC Huffman table 1"]
    pub huffenc_dc1_6: HUFFENC_DC1_6,
    #[doc = "0x7fc - JPEG encoder, DC Huffman table 1"]
    pub huffenc_dc1_7: HUFFENC_DC1_7,
}
#[doc = "JPEG codec configuration register 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jpeg_confr0](jpeg_confr0) module"]
pub type JPEG_CONFR0 = crate::Reg<u32, _JPEG_CONFR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JPEG_CONFR0;
#[doc = "`write(|w| ..)` method takes [jpeg_confr0::W](jpeg_confr0::W) writer structure"]
impl crate::Writable for JPEG_CONFR0 {}
#[doc = "JPEG codec configuration register 0"]
pub mod jpeg_confr0;
#[doc = "JPEG codec configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jpeg_confr1](jpeg_confr1) module"]
pub type JPEG_CONFR1 = crate::Reg<u32, _JPEG_CONFR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JPEG_CONFR1;
#[doc = "`read()` method returns [jpeg_confr1::R](jpeg_confr1::R) reader structure"]
impl crate::Readable for JPEG_CONFR1 {}
#[doc = "`write(|w| ..)` method takes [jpeg_confr1::W](jpeg_confr1::W) writer structure"]
impl crate::Writable for JPEG_CONFR1 {}
#[doc = "JPEG codec configuration register 1"]
pub mod jpeg_confr1;
#[doc = "JPEG codec configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jpeg_confr2](jpeg_confr2) module"]
pub type JPEG_CONFR2 = crate::Reg<u32, _JPEG_CONFR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JPEG_CONFR2;
#[doc = "`read()` method returns [jpeg_confr2::R](jpeg_confr2::R) reader structure"]
impl crate::Readable for JPEG_CONFR2 {}
#[doc = "`write(|w| ..)` method takes [jpeg_confr2::W](jpeg_confr2::W) writer structure"]
impl crate::Writable for JPEG_CONFR2 {}
#[doc = "JPEG codec configuration register 2"]
pub mod jpeg_confr2;
#[doc = "JPEG codec configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jpeg_confr3](jpeg_confr3) module"]
pub type JPEG_CONFR3 = crate::Reg<u32, _JPEG_CONFR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JPEG_CONFR3;
#[doc = "`read()` method returns [jpeg_confr3::R](jpeg_confr3::R) reader structure"]
impl crate::Readable for JPEG_CONFR3 {}
#[doc = "`write(|w| ..)` method takes [jpeg_confr3::W](jpeg_confr3::W) writer structure"]
impl crate::Writable for JPEG_CONFR3 {}
#[doc = "JPEG codec configuration register 3"]
pub mod jpeg_confr3;
#[doc = "JPEG codec configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jpeg_confr4](jpeg_confr4) module"]
pub type JPEG_CONFR4 = crate::Reg<u32, _JPEG_CONFR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JPEG_CONFR4;
#[doc = "`read()` method returns [jpeg_confr4::R](jpeg_confr4::R) reader structure"]
impl crate::Readable for JPEG_CONFR4 {}
#[doc = "`write(|w| ..)` method takes [jpeg_confr4::W](jpeg_confr4::W) writer structure"]
impl crate::Writable for JPEG_CONFR4 {}
#[doc = "JPEG codec configuration register 4"]
pub mod jpeg_confr4;
#[doc = "JPEG codec configuration register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jpeg_confr5](jpeg_confr5) module"]
pub type JPEG_CONFR5 = crate::Reg<u32, _JPEG_CONFR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JPEG_CONFR5;
#[doc = "`read()` method returns [jpeg_confr5::R](jpeg_confr5::R) reader structure"]
impl crate::Readable for JPEG_CONFR5 {}
#[doc = "`write(|w| ..)` method takes [jpeg_confr5::W](jpeg_confr5::W) writer structure"]
impl crate::Writable for JPEG_CONFR5 {}
#[doc = "JPEG codec configuration register 5"]
pub mod jpeg_confr5;
#[doc = "JPEG codec configuration register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jpeg_confr6](jpeg_confr6) module"]
pub type JPEG_CONFR6 = crate::Reg<u32, _JPEG_CONFR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JPEG_CONFR6;
#[doc = "`read()` method returns [jpeg_confr6::R](jpeg_confr6::R) reader structure"]
impl crate::Readable for JPEG_CONFR6 {}
#[doc = "`write(|w| ..)` method takes [jpeg_confr6::W](jpeg_confr6::W) writer structure"]
impl crate::Writable for JPEG_CONFR6 {}
#[doc = "JPEG codec configuration register 6"]
pub mod jpeg_confr6;
#[doc = "JPEG codec configuration register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jpeg_confr7](jpeg_confr7) module"]
pub type JPEG_CONFR7 = crate::Reg<u32, _JPEG_CONFR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JPEG_CONFR7;
#[doc = "`read()` method returns [jpeg_confr7::R](jpeg_confr7::R) reader structure"]
impl crate::Readable for JPEG_CONFR7 {}
#[doc = "`write(|w| ..)` method takes [jpeg_confr7::W](jpeg_confr7::W) writer structure"]
impl crate::Writable for JPEG_CONFR7 {}
#[doc = "JPEG codec configuration register 7"]
pub mod jpeg_confr7;
#[doc = "JPEG control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jpeg_cr](jpeg_cr) module"]
pub type JPEG_CR = crate::Reg<u32, _JPEG_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JPEG_CR;
#[doc = "`read()` method returns [jpeg_cr::R](jpeg_cr::R) reader structure"]
impl crate::Readable for JPEG_CR {}
#[doc = "`write(|w| ..)` method takes [jpeg_cr::W](jpeg_cr::W) writer structure"]
impl crate::Writable for JPEG_CR {}
#[doc = "JPEG control register"]
pub mod jpeg_cr;
#[doc = "JPEG status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jpeg_sr](jpeg_sr) module"]
pub type JPEG_SR = crate::Reg<u32, _JPEG_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JPEG_SR;
#[doc = "`read()` method returns [jpeg_sr::R](jpeg_sr::R) reader structure"]
impl crate::Readable for JPEG_SR {}
#[doc = "JPEG status register"]
pub mod jpeg_sr;
#[doc = "JPEG clear flag register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jpeg_cfr](jpeg_cfr) module"]
pub type JPEG_CFR = crate::Reg<u32, _JPEG_CFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JPEG_CFR;
#[doc = "`write(|w| ..)` method takes [jpeg_cfr::W](jpeg_cfr::W) writer structure"]
impl crate::Writable for JPEG_CFR {}
#[doc = "JPEG clear flag register"]
pub mod jpeg_cfr;
#[doc = "JPEG data input register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jpeg_dir](jpeg_dir) module"]
pub type JPEG_DIR = crate::Reg<u32, _JPEG_DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JPEG_DIR;
#[doc = "`write(|w| ..)` method takes [jpeg_dir::W](jpeg_dir::W) writer structure"]
impl crate::Writable for JPEG_DIR {}
#[doc = "JPEG data input register"]
pub mod jpeg_dir;
#[doc = "JPEG data output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jpeg_dor](jpeg_dor) module"]
pub type JPEG_DOR = crate::Reg<u32, _JPEG_DOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JPEG_DOR;
#[doc = "`read()` method returns [jpeg_dor::R](jpeg_dor::R) reader structure"]
impl crate::Readable for JPEG_DOR {}
#[doc = "JPEG data output register"]
pub mod jpeg_dor;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem0_0](qmem0_0) module"]
pub type QMEM0_0 = crate::Reg<u32, _QMEM0_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM0_0;
#[doc = "`read()` method returns [qmem0_0::R](qmem0_0::R) reader structure"]
impl crate::Readable for QMEM0_0 {}
#[doc = "`write(|w| ..)` method takes [qmem0_0::W](qmem0_0::W) writer structure"]
impl crate::Writable for QMEM0_0 {}
#[doc = "JPEG quantization tables"]
pub mod qmem0_0;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem0_1](qmem0_1) module"]
pub type QMEM0_1 = crate::Reg<u32, _QMEM0_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM0_1;
#[doc = "`read()` method returns [qmem0_1::R](qmem0_1::R) reader structure"]
impl crate::Readable for QMEM0_1 {}
#[doc = "`write(|w| ..)` method takes [qmem0_1::W](qmem0_1::W) writer structure"]
impl crate::Writable for QMEM0_1 {}
#[doc = "JPEG quantization tables"]
pub mod qmem0_1;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem0_2](qmem0_2) module"]
pub type QMEM0_2 = crate::Reg<u32, _QMEM0_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM0_2;
#[doc = "`read()` method returns [qmem0_2::R](qmem0_2::R) reader structure"]
impl crate::Readable for QMEM0_2 {}
#[doc = "`write(|w| ..)` method takes [qmem0_2::W](qmem0_2::W) writer structure"]
impl crate::Writable for QMEM0_2 {}
#[doc = "JPEG quantization tables"]
pub mod qmem0_2;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem0_3](qmem0_3) module"]
pub type QMEM0_3 = crate::Reg<u32, _QMEM0_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM0_3;
#[doc = "`read()` method returns [qmem0_3::R](qmem0_3::R) reader structure"]
impl crate::Readable for QMEM0_3 {}
#[doc = "`write(|w| ..)` method takes [qmem0_3::W](qmem0_3::W) writer structure"]
impl crate::Writable for QMEM0_3 {}
#[doc = "JPEG quantization tables"]
pub mod qmem0_3;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem0_4](qmem0_4) module"]
pub type QMEM0_4 = crate::Reg<u32, _QMEM0_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM0_4;
#[doc = "`read()` method returns [qmem0_4::R](qmem0_4::R) reader structure"]
impl crate::Readable for QMEM0_4 {}
#[doc = "`write(|w| ..)` method takes [qmem0_4::W](qmem0_4::W) writer structure"]
impl crate::Writable for QMEM0_4 {}
#[doc = "JPEG quantization tables"]
pub mod qmem0_4;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem0_5](qmem0_5) module"]
pub type QMEM0_5 = crate::Reg<u32, _QMEM0_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM0_5;
#[doc = "`read()` method returns [qmem0_5::R](qmem0_5::R) reader structure"]
impl crate::Readable for QMEM0_5 {}
#[doc = "`write(|w| ..)` method takes [qmem0_5::W](qmem0_5::W) writer structure"]
impl crate::Writable for QMEM0_5 {}
#[doc = "JPEG quantization tables"]
pub mod qmem0_5;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem0_6](qmem0_6) module"]
pub type QMEM0_6 = crate::Reg<u32, _QMEM0_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM0_6;
#[doc = "`read()` method returns [qmem0_6::R](qmem0_6::R) reader structure"]
impl crate::Readable for QMEM0_6 {}
#[doc = "`write(|w| ..)` method takes [qmem0_6::W](qmem0_6::W) writer structure"]
impl crate::Writable for QMEM0_6 {}
#[doc = "JPEG quantization tables"]
pub mod qmem0_6;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem0_7](qmem0_7) module"]
pub type QMEM0_7 = crate::Reg<u32, _QMEM0_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM0_7;
#[doc = "`read()` method returns [qmem0_7::R](qmem0_7::R) reader structure"]
impl crate::Readable for QMEM0_7 {}
#[doc = "`write(|w| ..)` method takes [qmem0_7::W](qmem0_7::W) writer structure"]
impl crate::Writable for QMEM0_7 {}
#[doc = "JPEG quantization tables"]
pub mod qmem0_7;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem0_8](qmem0_8) module"]
pub type QMEM0_8 = crate::Reg<u32, _QMEM0_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM0_8;
#[doc = "`read()` method returns [qmem0_8::R](qmem0_8::R) reader structure"]
impl crate::Readable for QMEM0_8 {}
#[doc = "`write(|w| ..)` method takes [qmem0_8::W](qmem0_8::W) writer structure"]
impl crate::Writable for QMEM0_8 {}
#[doc = "JPEG quantization tables"]
pub mod qmem0_8;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem0_9](qmem0_9) module"]
pub type QMEM0_9 = crate::Reg<u32, _QMEM0_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM0_9;
#[doc = "`read()` method returns [qmem0_9::R](qmem0_9::R) reader structure"]
impl crate::Readable for QMEM0_9 {}
#[doc = "`write(|w| ..)` method takes [qmem0_9::W](qmem0_9::W) writer structure"]
impl crate::Writable for QMEM0_9 {}
#[doc = "JPEG quantization tables"]
pub mod qmem0_9;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem0_10](qmem0_10) module"]
pub type QMEM0_10 = crate::Reg<u32, _QMEM0_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM0_10;
#[doc = "`read()` method returns [qmem0_10::R](qmem0_10::R) reader structure"]
impl crate::Readable for QMEM0_10 {}
#[doc = "`write(|w| ..)` method takes [qmem0_10::W](qmem0_10::W) writer structure"]
impl crate::Writable for QMEM0_10 {}
#[doc = "JPEG quantization tables"]
pub mod qmem0_10;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem0_11](qmem0_11) module"]
pub type QMEM0_11 = crate::Reg<u32, _QMEM0_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM0_11;
#[doc = "`read()` method returns [qmem0_11::R](qmem0_11::R) reader structure"]
impl crate::Readable for QMEM0_11 {}
#[doc = "`write(|w| ..)` method takes [qmem0_11::W](qmem0_11::W) writer structure"]
impl crate::Writable for QMEM0_11 {}
#[doc = "JPEG quantization tables"]
pub mod qmem0_11;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem0_12](qmem0_12) module"]
pub type QMEM0_12 = crate::Reg<u32, _QMEM0_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM0_12;
#[doc = "`read()` method returns [qmem0_12::R](qmem0_12::R) reader structure"]
impl crate::Readable for QMEM0_12 {}
#[doc = "`write(|w| ..)` method takes [qmem0_12::W](qmem0_12::W) writer structure"]
impl crate::Writable for QMEM0_12 {}
#[doc = "JPEG quantization tables"]
pub mod qmem0_12;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem0_13](qmem0_13) module"]
pub type QMEM0_13 = crate::Reg<u32, _QMEM0_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM0_13;
#[doc = "`read()` method returns [qmem0_13::R](qmem0_13::R) reader structure"]
impl crate::Readable for QMEM0_13 {}
#[doc = "`write(|w| ..)` method takes [qmem0_13::W](qmem0_13::W) writer structure"]
impl crate::Writable for QMEM0_13 {}
#[doc = "JPEG quantization tables"]
pub mod qmem0_13;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem0_14](qmem0_14) module"]
pub type QMEM0_14 = crate::Reg<u32, _QMEM0_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM0_14;
#[doc = "`read()` method returns [qmem0_14::R](qmem0_14::R) reader structure"]
impl crate::Readable for QMEM0_14 {}
#[doc = "`write(|w| ..)` method takes [qmem0_14::W](qmem0_14::W) writer structure"]
impl crate::Writable for QMEM0_14 {}
#[doc = "JPEG quantization tables"]
pub mod qmem0_14;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem0_15](qmem0_15) module"]
pub type QMEM0_15 = crate::Reg<u32, _QMEM0_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM0_15;
#[doc = "`read()` method returns [qmem0_15::R](qmem0_15::R) reader structure"]
impl crate::Readable for QMEM0_15 {}
#[doc = "`write(|w| ..)` method takes [qmem0_15::W](qmem0_15::W) writer structure"]
impl crate::Writable for QMEM0_15 {}
#[doc = "JPEG quantization tables"]
pub mod qmem0_15;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem1_0](qmem1_0) module"]
pub type QMEM1_0 = crate::Reg<u32, _QMEM1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM1_0;
#[doc = "`read()` method returns [qmem1_0::R](qmem1_0::R) reader structure"]
impl crate::Readable for QMEM1_0 {}
#[doc = "`write(|w| ..)` method takes [qmem1_0::W](qmem1_0::W) writer structure"]
impl crate::Writable for QMEM1_0 {}
#[doc = "JPEG quantization tables"]
pub mod qmem1_0;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem1_1](qmem1_1) module"]
pub type QMEM1_1 = crate::Reg<u32, _QMEM1_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM1_1;
#[doc = "`read()` method returns [qmem1_1::R](qmem1_1::R) reader structure"]
impl crate::Readable for QMEM1_1 {}
#[doc = "`write(|w| ..)` method takes [qmem1_1::W](qmem1_1::W) writer structure"]
impl crate::Writable for QMEM1_1 {}
#[doc = "JPEG quantization tables"]
pub mod qmem1_1;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem1_2](qmem1_2) module"]
pub type QMEM1_2 = crate::Reg<u32, _QMEM1_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM1_2;
#[doc = "`read()` method returns [qmem1_2::R](qmem1_2::R) reader structure"]
impl crate::Readable for QMEM1_2 {}
#[doc = "`write(|w| ..)` method takes [qmem1_2::W](qmem1_2::W) writer structure"]
impl crate::Writable for QMEM1_2 {}
#[doc = "JPEG quantization tables"]
pub mod qmem1_2;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem1_3](qmem1_3) module"]
pub type QMEM1_3 = crate::Reg<u32, _QMEM1_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM1_3;
#[doc = "`read()` method returns [qmem1_3::R](qmem1_3::R) reader structure"]
impl crate::Readable for QMEM1_3 {}
#[doc = "`write(|w| ..)` method takes [qmem1_3::W](qmem1_3::W) writer structure"]
impl crate::Writable for QMEM1_3 {}
#[doc = "JPEG quantization tables"]
pub mod qmem1_3;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem1_4](qmem1_4) module"]
pub type QMEM1_4 = crate::Reg<u32, _QMEM1_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM1_4;
#[doc = "`read()` method returns [qmem1_4::R](qmem1_4::R) reader structure"]
impl crate::Readable for QMEM1_4 {}
#[doc = "`write(|w| ..)` method takes [qmem1_4::W](qmem1_4::W) writer structure"]
impl crate::Writable for QMEM1_4 {}
#[doc = "JPEG quantization tables"]
pub mod qmem1_4;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem1_5](qmem1_5) module"]
pub type QMEM1_5 = crate::Reg<u32, _QMEM1_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM1_5;
#[doc = "`read()` method returns [qmem1_5::R](qmem1_5::R) reader structure"]
impl crate::Readable for QMEM1_5 {}
#[doc = "`write(|w| ..)` method takes [qmem1_5::W](qmem1_5::W) writer structure"]
impl crate::Writable for QMEM1_5 {}
#[doc = "JPEG quantization tables"]
pub mod qmem1_5;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem1_6](qmem1_6) module"]
pub type QMEM1_6 = crate::Reg<u32, _QMEM1_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM1_6;
#[doc = "`read()` method returns [qmem1_6::R](qmem1_6::R) reader structure"]
impl crate::Readable for QMEM1_6 {}
#[doc = "`write(|w| ..)` method takes [qmem1_6::W](qmem1_6::W) writer structure"]
impl crate::Writable for QMEM1_6 {}
#[doc = "JPEG quantization tables"]
pub mod qmem1_6;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem1_7](qmem1_7) module"]
pub type QMEM1_7 = crate::Reg<u32, _QMEM1_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM1_7;
#[doc = "`read()` method returns [qmem1_7::R](qmem1_7::R) reader structure"]
impl crate::Readable for QMEM1_7 {}
#[doc = "`write(|w| ..)` method takes [qmem1_7::W](qmem1_7::W) writer structure"]
impl crate::Writable for QMEM1_7 {}
#[doc = "JPEG quantization tables"]
pub mod qmem1_7;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem1_8](qmem1_8) module"]
pub type QMEM1_8 = crate::Reg<u32, _QMEM1_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM1_8;
#[doc = "`read()` method returns [qmem1_8::R](qmem1_8::R) reader structure"]
impl crate::Readable for QMEM1_8 {}
#[doc = "`write(|w| ..)` method takes [qmem1_8::W](qmem1_8::W) writer structure"]
impl crate::Writable for QMEM1_8 {}
#[doc = "JPEG quantization tables"]
pub mod qmem1_8;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem1_9](qmem1_9) module"]
pub type QMEM1_9 = crate::Reg<u32, _QMEM1_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM1_9;
#[doc = "`read()` method returns [qmem1_9::R](qmem1_9::R) reader structure"]
impl crate::Readable for QMEM1_9 {}
#[doc = "`write(|w| ..)` method takes [qmem1_9::W](qmem1_9::W) writer structure"]
impl crate::Writable for QMEM1_9 {}
#[doc = "JPEG quantization tables"]
pub mod qmem1_9;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem1_10](qmem1_10) module"]
pub type QMEM1_10 = crate::Reg<u32, _QMEM1_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM1_10;
#[doc = "`read()` method returns [qmem1_10::R](qmem1_10::R) reader structure"]
impl crate::Readable for QMEM1_10 {}
#[doc = "`write(|w| ..)` method takes [qmem1_10::W](qmem1_10::W) writer structure"]
impl crate::Writable for QMEM1_10 {}
#[doc = "JPEG quantization tables"]
pub mod qmem1_10;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem1_11](qmem1_11) module"]
pub type QMEM1_11 = crate::Reg<u32, _QMEM1_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM1_11;
#[doc = "`read()` method returns [qmem1_11::R](qmem1_11::R) reader structure"]
impl crate::Readable for QMEM1_11 {}
#[doc = "`write(|w| ..)` method takes [qmem1_11::W](qmem1_11::W) writer structure"]
impl crate::Writable for QMEM1_11 {}
#[doc = "JPEG quantization tables"]
pub mod qmem1_11;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem1_12](qmem1_12) module"]
pub type QMEM1_12 = crate::Reg<u32, _QMEM1_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM1_12;
#[doc = "`read()` method returns [qmem1_12::R](qmem1_12::R) reader structure"]
impl crate::Readable for QMEM1_12 {}
#[doc = "`write(|w| ..)` method takes [qmem1_12::W](qmem1_12::W) writer structure"]
impl crate::Writable for QMEM1_12 {}
#[doc = "JPEG quantization tables"]
pub mod qmem1_12;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem1_13](qmem1_13) module"]
pub type QMEM1_13 = crate::Reg<u32, _QMEM1_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM1_13;
#[doc = "`read()` method returns [qmem1_13::R](qmem1_13::R) reader structure"]
impl crate::Readable for QMEM1_13 {}
#[doc = "`write(|w| ..)` method takes [qmem1_13::W](qmem1_13::W) writer structure"]
impl crate::Writable for QMEM1_13 {}
#[doc = "JPEG quantization tables"]
pub mod qmem1_13;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem1_14](qmem1_14) module"]
pub type QMEM1_14 = crate::Reg<u32, _QMEM1_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM1_14;
#[doc = "`read()` method returns [qmem1_14::R](qmem1_14::R) reader structure"]
impl crate::Readable for QMEM1_14 {}
#[doc = "`write(|w| ..)` method takes [qmem1_14::W](qmem1_14::W) writer structure"]
impl crate::Writable for QMEM1_14 {}
#[doc = "JPEG quantization tables"]
pub mod qmem1_14;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem1_15](qmem1_15) module"]
pub type QMEM1_15 = crate::Reg<u32, _QMEM1_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM1_15;
#[doc = "`read()` method returns [qmem1_15::R](qmem1_15::R) reader structure"]
impl crate::Readable for QMEM1_15 {}
#[doc = "`write(|w| ..)` method takes [qmem1_15::W](qmem1_15::W) writer structure"]
impl crate::Writable for QMEM1_15 {}
#[doc = "JPEG quantization tables"]
pub mod qmem1_15;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem2_0](qmem2_0) module"]
pub type QMEM2_0 = crate::Reg<u32, _QMEM2_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM2_0;
#[doc = "`read()` method returns [qmem2_0::R](qmem2_0::R) reader structure"]
impl crate::Readable for QMEM2_0 {}
#[doc = "`write(|w| ..)` method takes [qmem2_0::W](qmem2_0::W) writer structure"]
impl crate::Writable for QMEM2_0 {}
#[doc = "JPEG quantization tables"]
pub mod qmem2_0;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem2_1](qmem2_1) module"]
pub type QMEM2_1 = crate::Reg<u32, _QMEM2_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM2_1;
#[doc = "`read()` method returns [qmem2_1::R](qmem2_1::R) reader structure"]
impl crate::Readable for QMEM2_1 {}
#[doc = "`write(|w| ..)` method takes [qmem2_1::W](qmem2_1::W) writer structure"]
impl crate::Writable for QMEM2_1 {}
#[doc = "JPEG quantization tables"]
pub mod qmem2_1;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem2_2](qmem2_2) module"]
pub type QMEM2_2 = crate::Reg<u32, _QMEM2_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM2_2;
#[doc = "`read()` method returns [qmem2_2::R](qmem2_2::R) reader structure"]
impl crate::Readable for QMEM2_2 {}
#[doc = "`write(|w| ..)` method takes [qmem2_2::W](qmem2_2::W) writer structure"]
impl crate::Writable for QMEM2_2 {}
#[doc = "JPEG quantization tables"]
pub mod qmem2_2;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem2_3](qmem2_3) module"]
pub type QMEM2_3 = crate::Reg<u32, _QMEM2_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM2_3;
#[doc = "`read()` method returns [qmem2_3::R](qmem2_3::R) reader structure"]
impl crate::Readable for QMEM2_3 {}
#[doc = "`write(|w| ..)` method takes [qmem2_3::W](qmem2_3::W) writer structure"]
impl crate::Writable for QMEM2_3 {}
#[doc = "JPEG quantization tables"]
pub mod qmem2_3;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem2_4](qmem2_4) module"]
pub type QMEM2_4 = crate::Reg<u32, _QMEM2_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM2_4;
#[doc = "`read()` method returns [qmem2_4::R](qmem2_4::R) reader structure"]
impl crate::Readable for QMEM2_4 {}
#[doc = "`write(|w| ..)` method takes [qmem2_4::W](qmem2_4::W) writer structure"]
impl crate::Writable for QMEM2_4 {}
#[doc = "JPEG quantization tables"]
pub mod qmem2_4;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem2_5](qmem2_5) module"]
pub type QMEM2_5 = crate::Reg<u32, _QMEM2_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM2_5;
#[doc = "`read()` method returns [qmem2_5::R](qmem2_5::R) reader structure"]
impl crate::Readable for QMEM2_5 {}
#[doc = "`write(|w| ..)` method takes [qmem2_5::W](qmem2_5::W) writer structure"]
impl crate::Writable for QMEM2_5 {}
#[doc = "JPEG quantization tables"]
pub mod qmem2_5;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem2_6](qmem2_6) module"]
pub type QMEM2_6 = crate::Reg<u32, _QMEM2_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM2_6;
#[doc = "`read()` method returns [qmem2_6::R](qmem2_6::R) reader structure"]
impl crate::Readable for QMEM2_6 {}
#[doc = "`write(|w| ..)` method takes [qmem2_6::W](qmem2_6::W) writer structure"]
impl crate::Writable for QMEM2_6 {}
#[doc = "JPEG quantization tables"]
pub mod qmem2_6;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem2_7](qmem2_7) module"]
pub type QMEM2_7 = crate::Reg<u32, _QMEM2_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM2_7;
#[doc = "`read()` method returns [qmem2_7::R](qmem2_7::R) reader structure"]
impl crate::Readable for QMEM2_7 {}
#[doc = "`write(|w| ..)` method takes [qmem2_7::W](qmem2_7::W) writer structure"]
impl crate::Writable for QMEM2_7 {}
#[doc = "JPEG quantization tables"]
pub mod qmem2_7;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem2_8](qmem2_8) module"]
pub type QMEM2_8 = crate::Reg<u32, _QMEM2_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM2_8;
#[doc = "`read()` method returns [qmem2_8::R](qmem2_8::R) reader structure"]
impl crate::Readable for QMEM2_8 {}
#[doc = "`write(|w| ..)` method takes [qmem2_8::W](qmem2_8::W) writer structure"]
impl crate::Writable for QMEM2_8 {}
#[doc = "JPEG quantization tables"]
pub mod qmem2_8;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem2_9](qmem2_9) module"]
pub type QMEM2_9 = crate::Reg<u32, _QMEM2_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM2_9;
#[doc = "`read()` method returns [qmem2_9::R](qmem2_9::R) reader structure"]
impl crate::Readable for QMEM2_9 {}
#[doc = "`write(|w| ..)` method takes [qmem2_9::W](qmem2_9::W) writer structure"]
impl crate::Writable for QMEM2_9 {}
#[doc = "JPEG quantization tables"]
pub mod qmem2_9;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem2_10](qmem2_10) module"]
pub type QMEM2_10 = crate::Reg<u32, _QMEM2_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM2_10;
#[doc = "`read()` method returns [qmem2_10::R](qmem2_10::R) reader structure"]
impl crate::Readable for QMEM2_10 {}
#[doc = "`write(|w| ..)` method takes [qmem2_10::W](qmem2_10::W) writer structure"]
impl crate::Writable for QMEM2_10 {}
#[doc = "JPEG quantization tables"]
pub mod qmem2_10;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem2_11](qmem2_11) module"]
pub type QMEM2_11 = crate::Reg<u32, _QMEM2_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM2_11;
#[doc = "`read()` method returns [qmem2_11::R](qmem2_11::R) reader structure"]
impl crate::Readable for QMEM2_11 {}
#[doc = "`write(|w| ..)` method takes [qmem2_11::W](qmem2_11::W) writer structure"]
impl crate::Writable for QMEM2_11 {}
#[doc = "JPEG quantization tables"]
pub mod qmem2_11;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem2_12](qmem2_12) module"]
pub type QMEM2_12 = crate::Reg<u32, _QMEM2_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM2_12;
#[doc = "`read()` method returns [qmem2_12::R](qmem2_12::R) reader structure"]
impl crate::Readable for QMEM2_12 {}
#[doc = "`write(|w| ..)` method takes [qmem2_12::W](qmem2_12::W) writer structure"]
impl crate::Writable for QMEM2_12 {}
#[doc = "JPEG quantization tables"]
pub mod qmem2_12;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem2_13](qmem2_13) module"]
pub type QMEM2_13 = crate::Reg<u32, _QMEM2_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM2_13;
#[doc = "`read()` method returns [qmem2_13::R](qmem2_13::R) reader structure"]
impl crate::Readable for QMEM2_13 {}
#[doc = "`write(|w| ..)` method takes [qmem2_13::W](qmem2_13::W) writer structure"]
impl crate::Writable for QMEM2_13 {}
#[doc = "JPEG quantization tables"]
pub mod qmem2_13;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem2_14](qmem2_14) module"]
pub type QMEM2_14 = crate::Reg<u32, _QMEM2_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM2_14;
#[doc = "`read()` method returns [qmem2_14::R](qmem2_14::R) reader structure"]
impl crate::Readable for QMEM2_14 {}
#[doc = "`write(|w| ..)` method takes [qmem2_14::W](qmem2_14::W) writer structure"]
impl crate::Writable for QMEM2_14 {}
#[doc = "JPEG quantization tables"]
pub mod qmem2_14;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem2_15](qmem2_15) module"]
pub type QMEM2_15 = crate::Reg<u32, _QMEM2_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM2_15;
#[doc = "`read()` method returns [qmem2_15::R](qmem2_15::R) reader structure"]
impl crate::Readable for QMEM2_15 {}
#[doc = "`write(|w| ..)` method takes [qmem2_15::W](qmem2_15::W) writer structure"]
impl crate::Writable for QMEM2_15 {}
#[doc = "JPEG quantization tables"]
pub mod qmem2_15;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem3_0](qmem3_0) module"]
pub type QMEM3_0 = crate::Reg<u32, _QMEM3_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM3_0;
#[doc = "`read()` method returns [qmem3_0::R](qmem3_0::R) reader structure"]
impl crate::Readable for QMEM3_0 {}
#[doc = "`write(|w| ..)` method takes [qmem3_0::W](qmem3_0::W) writer structure"]
impl crate::Writable for QMEM3_0 {}
#[doc = "JPEG quantization tables"]
pub mod qmem3_0;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem3_1](qmem3_1) module"]
pub type QMEM3_1 = crate::Reg<u32, _QMEM3_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM3_1;
#[doc = "`read()` method returns [qmem3_1::R](qmem3_1::R) reader structure"]
impl crate::Readable for QMEM3_1 {}
#[doc = "`write(|w| ..)` method takes [qmem3_1::W](qmem3_1::W) writer structure"]
impl crate::Writable for QMEM3_1 {}
#[doc = "JPEG quantization tables"]
pub mod qmem3_1;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem3_2](qmem3_2) module"]
pub type QMEM3_2 = crate::Reg<u32, _QMEM3_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM3_2;
#[doc = "`read()` method returns [qmem3_2::R](qmem3_2::R) reader structure"]
impl crate::Readable for QMEM3_2 {}
#[doc = "`write(|w| ..)` method takes [qmem3_2::W](qmem3_2::W) writer structure"]
impl crate::Writable for QMEM3_2 {}
#[doc = "JPEG quantization tables"]
pub mod qmem3_2;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem3_3](qmem3_3) module"]
pub type QMEM3_3 = crate::Reg<u32, _QMEM3_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM3_3;
#[doc = "`read()` method returns [qmem3_3::R](qmem3_3::R) reader structure"]
impl crate::Readable for QMEM3_3 {}
#[doc = "`write(|w| ..)` method takes [qmem3_3::W](qmem3_3::W) writer structure"]
impl crate::Writable for QMEM3_3 {}
#[doc = "JPEG quantization tables"]
pub mod qmem3_3;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem3_4](qmem3_4) module"]
pub type QMEM3_4 = crate::Reg<u32, _QMEM3_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM3_4;
#[doc = "`read()` method returns [qmem3_4::R](qmem3_4::R) reader structure"]
impl crate::Readable for QMEM3_4 {}
#[doc = "`write(|w| ..)` method takes [qmem3_4::W](qmem3_4::W) writer structure"]
impl crate::Writable for QMEM3_4 {}
#[doc = "JPEG quantization tables"]
pub mod qmem3_4;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem3_5](qmem3_5) module"]
pub type QMEM3_5 = crate::Reg<u32, _QMEM3_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM3_5;
#[doc = "`read()` method returns [qmem3_5::R](qmem3_5::R) reader structure"]
impl crate::Readable for QMEM3_5 {}
#[doc = "`write(|w| ..)` method takes [qmem3_5::W](qmem3_5::W) writer structure"]
impl crate::Writable for QMEM3_5 {}
#[doc = "JPEG quantization tables"]
pub mod qmem3_5;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem3_6](qmem3_6) module"]
pub type QMEM3_6 = crate::Reg<u32, _QMEM3_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM3_6;
#[doc = "`read()` method returns [qmem3_6::R](qmem3_6::R) reader structure"]
impl crate::Readable for QMEM3_6 {}
#[doc = "`write(|w| ..)` method takes [qmem3_6::W](qmem3_6::W) writer structure"]
impl crate::Writable for QMEM3_6 {}
#[doc = "JPEG quantization tables"]
pub mod qmem3_6;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem3_7](qmem3_7) module"]
pub type QMEM3_7 = crate::Reg<u32, _QMEM3_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM3_7;
#[doc = "`read()` method returns [qmem3_7::R](qmem3_7::R) reader structure"]
impl crate::Readable for QMEM3_7 {}
#[doc = "`write(|w| ..)` method takes [qmem3_7::W](qmem3_7::W) writer structure"]
impl crate::Writable for QMEM3_7 {}
#[doc = "JPEG quantization tables"]
pub mod qmem3_7;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem3_8](qmem3_8) module"]
pub type QMEM3_8 = crate::Reg<u32, _QMEM3_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM3_8;
#[doc = "`read()` method returns [qmem3_8::R](qmem3_8::R) reader structure"]
impl crate::Readable for QMEM3_8 {}
#[doc = "`write(|w| ..)` method takes [qmem3_8::W](qmem3_8::W) writer structure"]
impl crate::Writable for QMEM3_8 {}
#[doc = "JPEG quantization tables"]
pub mod qmem3_8;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem3_9](qmem3_9) module"]
pub type QMEM3_9 = crate::Reg<u32, _QMEM3_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM3_9;
#[doc = "`read()` method returns [qmem3_9::R](qmem3_9::R) reader structure"]
impl crate::Readable for QMEM3_9 {}
#[doc = "`write(|w| ..)` method takes [qmem3_9::W](qmem3_9::W) writer structure"]
impl crate::Writable for QMEM3_9 {}
#[doc = "JPEG quantization tables"]
pub mod qmem3_9;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem3_10](qmem3_10) module"]
pub type QMEM3_10 = crate::Reg<u32, _QMEM3_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM3_10;
#[doc = "`read()` method returns [qmem3_10::R](qmem3_10::R) reader structure"]
impl crate::Readable for QMEM3_10 {}
#[doc = "`write(|w| ..)` method takes [qmem3_10::W](qmem3_10::W) writer structure"]
impl crate::Writable for QMEM3_10 {}
#[doc = "JPEG quantization tables"]
pub mod qmem3_10;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem3_11](qmem3_11) module"]
pub type QMEM3_11 = crate::Reg<u32, _QMEM3_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM3_11;
#[doc = "`read()` method returns [qmem3_11::R](qmem3_11::R) reader structure"]
impl crate::Readable for QMEM3_11 {}
#[doc = "`write(|w| ..)` method takes [qmem3_11::W](qmem3_11::W) writer structure"]
impl crate::Writable for QMEM3_11 {}
#[doc = "JPEG quantization tables"]
pub mod qmem3_11;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem3_12](qmem3_12) module"]
pub type QMEM3_12 = crate::Reg<u32, _QMEM3_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM3_12;
#[doc = "`read()` method returns [qmem3_12::R](qmem3_12::R) reader structure"]
impl crate::Readable for QMEM3_12 {}
#[doc = "`write(|w| ..)` method takes [qmem3_12::W](qmem3_12::W) writer structure"]
impl crate::Writable for QMEM3_12 {}
#[doc = "JPEG quantization tables"]
pub mod qmem3_12;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem3_13](qmem3_13) module"]
pub type QMEM3_13 = crate::Reg<u32, _QMEM3_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM3_13;
#[doc = "`read()` method returns [qmem3_13::R](qmem3_13::R) reader structure"]
impl crate::Readable for QMEM3_13 {}
#[doc = "`write(|w| ..)` method takes [qmem3_13::W](qmem3_13::W) writer structure"]
impl crate::Writable for QMEM3_13 {}
#[doc = "JPEG quantization tables"]
pub mod qmem3_13;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem3_14](qmem3_14) module"]
pub type QMEM3_14 = crate::Reg<u32, _QMEM3_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM3_14;
#[doc = "`read()` method returns [qmem3_14::R](qmem3_14::R) reader structure"]
impl crate::Readable for QMEM3_14 {}
#[doc = "`write(|w| ..)` method takes [qmem3_14::W](qmem3_14::W) writer structure"]
impl crate::Writable for QMEM3_14 {}
#[doc = "JPEG quantization tables"]
pub mod qmem3_14;
#[doc = "JPEG quantization tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qmem3_15](qmem3_15) module"]
pub type QMEM3_15 = crate::Reg<u32, _QMEM3_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMEM3_15;
#[doc = "`read()` method returns [qmem3_15::R](qmem3_15::R) reader structure"]
impl crate::Readable for QMEM3_15 {}
#[doc = "`write(|w| ..)` method takes [qmem3_15::W](qmem3_15::W) writer structure"]
impl crate::Writable for QMEM3_15 {}
#[doc = "JPEG quantization tables"]
pub mod qmem3_15;
#[doc = "JPEG HuffMin tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffmin_0](huffmin_0) module"]
pub type HUFFMIN_0 = crate::Reg<u32, _HUFFMIN_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFMIN_0;
#[doc = "`read()` method returns [huffmin_0::R](huffmin_0::R) reader structure"]
impl crate::Readable for HUFFMIN_0 {}
#[doc = "`write(|w| ..)` method takes [huffmin_0::W](huffmin_0::W) writer structure"]
impl crate::Writable for HUFFMIN_0 {}
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_0;
#[doc = "JPEG HuffMin tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffmin_1](huffmin_1) module"]
pub type HUFFMIN_1 = crate::Reg<u32, _HUFFMIN_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFMIN_1;
#[doc = "`read()` method returns [huffmin_1::R](huffmin_1::R) reader structure"]
impl crate::Readable for HUFFMIN_1 {}
#[doc = "`write(|w| ..)` method takes [huffmin_1::W](huffmin_1::W) writer structure"]
impl crate::Writable for HUFFMIN_1 {}
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_1;
#[doc = "JPEG HuffMin tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffmin_2](huffmin_2) module"]
pub type HUFFMIN_2 = crate::Reg<u32, _HUFFMIN_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFMIN_2;
#[doc = "`read()` method returns [huffmin_2::R](huffmin_2::R) reader structure"]
impl crate::Readable for HUFFMIN_2 {}
#[doc = "`write(|w| ..)` method takes [huffmin_2::W](huffmin_2::W) writer structure"]
impl crate::Writable for HUFFMIN_2 {}
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_2;
#[doc = "JPEG HuffMin tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffmin_3](huffmin_3) module"]
pub type HUFFMIN_3 = crate::Reg<u32, _HUFFMIN_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFMIN_3;
#[doc = "`read()` method returns [huffmin_3::R](huffmin_3::R) reader structure"]
impl crate::Readable for HUFFMIN_3 {}
#[doc = "`write(|w| ..)` method takes [huffmin_3::W](huffmin_3::W) writer structure"]
impl crate::Writable for HUFFMIN_3 {}
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_3;
#[doc = "JPEG HuffMin tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffmin_4](huffmin_4) module"]
pub type HUFFMIN_4 = crate::Reg<u32, _HUFFMIN_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFMIN_4;
#[doc = "`read()` method returns [huffmin_4::R](huffmin_4::R) reader structure"]
impl crate::Readable for HUFFMIN_4 {}
#[doc = "`write(|w| ..)` method takes [huffmin_4::W](huffmin_4::W) writer structure"]
impl crate::Writable for HUFFMIN_4 {}
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_4;
#[doc = "JPEG HuffMin tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffmin_5](huffmin_5) module"]
pub type HUFFMIN_5 = crate::Reg<u32, _HUFFMIN_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFMIN_5;
#[doc = "`read()` method returns [huffmin_5::R](huffmin_5::R) reader structure"]
impl crate::Readable for HUFFMIN_5 {}
#[doc = "`write(|w| ..)` method takes [huffmin_5::W](huffmin_5::W) writer structure"]
impl crate::Writable for HUFFMIN_5 {}
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_5;
#[doc = "JPEG HuffMin tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffmin_6](huffmin_6) module"]
pub type HUFFMIN_6 = crate::Reg<u32, _HUFFMIN_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFMIN_6;
#[doc = "`read()` method returns [huffmin_6::R](huffmin_6::R) reader structure"]
impl crate::Readable for HUFFMIN_6 {}
#[doc = "`write(|w| ..)` method takes [huffmin_6::W](huffmin_6::W) writer structure"]
impl crate::Writable for HUFFMIN_6 {}
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_6;
#[doc = "JPEG HuffMin tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffmin_7](huffmin_7) module"]
pub type HUFFMIN_7 = crate::Reg<u32, _HUFFMIN_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFMIN_7;
#[doc = "`read()` method returns [huffmin_7::R](huffmin_7::R) reader structure"]
impl crate::Readable for HUFFMIN_7 {}
#[doc = "`write(|w| ..)` method takes [huffmin_7::W](huffmin_7::W) writer structure"]
impl crate::Writable for HUFFMIN_7 {}
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_7;
#[doc = "JPEG HuffMin tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffmin_8](huffmin_8) module"]
pub type HUFFMIN_8 = crate::Reg<u32, _HUFFMIN_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFMIN_8;
#[doc = "`read()` method returns [huffmin_8::R](huffmin_8::R) reader structure"]
impl crate::Readable for HUFFMIN_8 {}
#[doc = "`write(|w| ..)` method takes [huffmin_8::W](huffmin_8::W) writer structure"]
impl crate::Writable for HUFFMIN_8 {}
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_8;
#[doc = "JPEG HuffMin tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffmin_9](huffmin_9) module"]
pub type HUFFMIN_9 = crate::Reg<u32, _HUFFMIN_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFMIN_9;
#[doc = "`read()` method returns [huffmin_9::R](huffmin_9::R) reader structure"]
impl crate::Readable for HUFFMIN_9 {}
#[doc = "`write(|w| ..)` method takes [huffmin_9::W](huffmin_9::W) writer structure"]
impl crate::Writable for HUFFMIN_9 {}
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_9;
#[doc = "JPEG HuffMin tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffmin_10](huffmin_10) module"]
pub type HUFFMIN_10 = crate::Reg<u32, _HUFFMIN_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFMIN_10;
#[doc = "`read()` method returns [huffmin_10::R](huffmin_10::R) reader structure"]
impl crate::Readable for HUFFMIN_10 {}
#[doc = "`write(|w| ..)` method takes [huffmin_10::W](huffmin_10::W) writer structure"]
impl crate::Writable for HUFFMIN_10 {}
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_10;
#[doc = "JPEG HuffMin tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffmin_11](huffmin_11) module"]
pub type HUFFMIN_11 = crate::Reg<u32, _HUFFMIN_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFMIN_11;
#[doc = "`read()` method returns [huffmin_11::R](huffmin_11::R) reader structure"]
impl crate::Readable for HUFFMIN_11 {}
#[doc = "`write(|w| ..)` method takes [huffmin_11::W](huffmin_11::W) writer structure"]
impl crate::Writable for HUFFMIN_11 {}
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_11;
#[doc = "JPEG HuffMin tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffmin_12](huffmin_12) module"]
pub type HUFFMIN_12 = crate::Reg<u32, _HUFFMIN_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFMIN_12;
#[doc = "`read()` method returns [huffmin_12::R](huffmin_12::R) reader structure"]
impl crate::Readable for HUFFMIN_12 {}
#[doc = "`write(|w| ..)` method takes [huffmin_12::W](huffmin_12::W) writer structure"]
impl crate::Writable for HUFFMIN_12 {}
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_12;
#[doc = "JPEG HuffMin tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffmin_13](huffmin_13) module"]
pub type HUFFMIN_13 = crate::Reg<u32, _HUFFMIN_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFMIN_13;
#[doc = "`read()` method returns [huffmin_13::R](huffmin_13::R) reader structure"]
impl crate::Readable for HUFFMIN_13 {}
#[doc = "`write(|w| ..)` method takes [huffmin_13::W](huffmin_13::W) writer structure"]
impl crate::Writable for HUFFMIN_13 {}
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_13;
#[doc = "JPEG HuffMin tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffmin_14](huffmin_14) module"]
pub type HUFFMIN_14 = crate::Reg<u32, _HUFFMIN_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFMIN_14;
#[doc = "`read()` method returns [huffmin_14::R](huffmin_14::R) reader structure"]
impl crate::Readable for HUFFMIN_14 {}
#[doc = "`write(|w| ..)` method takes [huffmin_14::W](huffmin_14::W) writer structure"]
impl crate::Writable for HUFFMIN_14 {}
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_14;
#[doc = "JPEG HuffMin tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffmin_15](huffmin_15) module"]
pub type HUFFMIN_15 = crate::Reg<u32, _HUFFMIN_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFMIN_15;
#[doc = "`read()` method returns [huffmin_15::R](huffmin_15::R) reader structure"]
impl crate::Readable for HUFFMIN_15 {}
#[doc = "`write(|w| ..)` method takes [huffmin_15::W](huffmin_15::W) writer structure"]
impl crate::Writable for HUFFMIN_15 {}
#[doc = "JPEG HuffMin tables"]
pub mod huffmin_15;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase0](huffbase0) module"]
pub type HUFFBASE0 = crate::Reg<u32, _HUFFBASE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE0;
#[doc = "`read()` method returns [huffbase0::R](huffbase0::R) reader structure"]
impl crate::Readable for HUFFBASE0 {}
#[doc = "`write(|w| ..)` method takes [huffbase0::W](huffbase0::W) writer structure"]
impl crate::Writable for HUFFBASE0 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase0;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase1](huffbase1) module"]
pub type HUFFBASE1 = crate::Reg<u32, _HUFFBASE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE1;
#[doc = "`read()` method returns [huffbase1::R](huffbase1::R) reader structure"]
impl crate::Readable for HUFFBASE1 {}
#[doc = "`write(|w| ..)` method takes [huffbase1::W](huffbase1::W) writer structure"]
impl crate::Writable for HUFFBASE1 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase1;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase2](huffbase2) module"]
pub type HUFFBASE2 = crate::Reg<u32, _HUFFBASE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE2;
#[doc = "`read()` method returns [huffbase2::R](huffbase2::R) reader structure"]
impl crate::Readable for HUFFBASE2 {}
#[doc = "`write(|w| ..)` method takes [huffbase2::W](huffbase2::W) writer structure"]
impl crate::Writable for HUFFBASE2 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase2;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase3](huffbase3) module"]
pub type HUFFBASE3 = crate::Reg<u32, _HUFFBASE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE3;
#[doc = "`read()` method returns [huffbase3::R](huffbase3::R) reader structure"]
impl crate::Readable for HUFFBASE3 {}
#[doc = "`write(|w| ..)` method takes [huffbase3::W](huffbase3::W) writer structure"]
impl crate::Writable for HUFFBASE3 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase3;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase4](huffbase4) module"]
pub type HUFFBASE4 = crate::Reg<u32, _HUFFBASE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE4;
#[doc = "`read()` method returns [huffbase4::R](huffbase4::R) reader structure"]
impl crate::Readable for HUFFBASE4 {}
#[doc = "`write(|w| ..)` method takes [huffbase4::W](huffbase4::W) writer structure"]
impl crate::Writable for HUFFBASE4 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase4;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase5](huffbase5) module"]
pub type HUFFBASE5 = crate::Reg<u32, _HUFFBASE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE5;
#[doc = "`read()` method returns [huffbase5::R](huffbase5::R) reader structure"]
impl crate::Readable for HUFFBASE5 {}
#[doc = "`write(|w| ..)` method takes [huffbase5::W](huffbase5::W) writer structure"]
impl crate::Writable for HUFFBASE5 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase5;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase6](huffbase6) module"]
pub type HUFFBASE6 = crate::Reg<u32, _HUFFBASE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE6;
#[doc = "`read()` method returns [huffbase6::R](huffbase6::R) reader structure"]
impl crate::Readable for HUFFBASE6 {}
#[doc = "`write(|w| ..)` method takes [huffbase6::W](huffbase6::W) writer structure"]
impl crate::Writable for HUFFBASE6 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase6;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase7](huffbase7) module"]
pub type HUFFBASE7 = crate::Reg<u32, _HUFFBASE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE7;
#[doc = "`read()` method returns [huffbase7::R](huffbase7::R) reader structure"]
impl crate::Readable for HUFFBASE7 {}
#[doc = "`write(|w| ..)` method takes [huffbase7::W](huffbase7::W) writer structure"]
impl crate::Writable for HUFFBASE7 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase7;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase8](huffbase8) module"]
pub type HUFFBASE8 = crate::Reg<u32, _HUFFBASE8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE8;
#[doc = "`read()` method returns [huffbase8::R](huffbase8::R) reader structure"]
impl crate::Readable for HUFFBASE8 {}
#[doc = "`write(|w| ..)` method takes [huffbase8::W](huffbase8::W) writer structure"]
impl crate::Writable for HUFFBASE8 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase8;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase9](huffbase9) module"]
pub type HUFFBASE9 = crate::Reg<u32, _HUFFBASE9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE9;
#[doc = "`read()` method returns [huffbase9::R](huffbase9::R) reader structure"]
impl crate::Readable for HUFFBASE9 {}
#[doc = "`write(|w| ..)` method takes [huffbase9::W](huffbase9::W) writer structure"]
impl crate::Writable for HUFFBASE9 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase9;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase10](huffbase10) module"]
pub type HUFFBASE10 = crate::Reg<u32, _HUFFBASE10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE10;
#[doc = "`read()` method returns [huffbase10::R](huffbase10::R) reader structure"]
impl crate::Readable for HUFFBASE10 {}
#[doc = "`write(|w| ..)` method takes [huffbase10::W](huffbase10::W) writer structure"]
impl crate::Writable for HUFFBASE10 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase10;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase11](huffbase11) module"]
pub type HUFFBASE11 = crate::Reg<u32, _HUFFBASE11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE11;
#[doc = "`read()` method returns [huffbase11::R](huffbase11::R) reader structure"]
impl crate::Readable for HUFFBASE11 {}
#[doc = "`write(|w| ..)` method takes [huffbase11::W](huffbase11::W) writer structure"]
impl crate::Writable for HUFFBASE11 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase11;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase12](huffbase12) module"]
pub type HUFFBASE12 = crate::Reg<u32, _HUFFBASE12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE12;
#[doc = "`read()` method returns [huffbase12::R](huffbase12::R) reader structure"]
impl crate::Readable for HUFFBASE12 {}
#[doc = "`write(|w| ..)` method takes [huffbase12::W](huffbase12::W) writer structure"]
impl crate::Writable for HUFFBASE12 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase12;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase13](huffbase13) module"]
pub type HUFFBASE13 = crate::Reg<u32, _HUFFBASE13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE13;
#[doc = "`read()` method returns [huffbase13::R](huffbase13::R) reader structure"]
impl crate::Readable for HUFFBASE13 {}
#[doc = "`write(|w| ..)` method takes [huffbase13::W](huffbase13::W) writer structure"]
impl crate::Writable for HUFFBASE13 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase13;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase14](huffbase14) module"]
pub type HUFFBASE14 = crate::Reg<u32, _HUFFBASE14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE14;
#[doc = "`read()` method returns [huffbase14::R](huffbase14::R) reader structure"]
impl crate::Readable for HUFFBASE14 {}
#[doc = "`write(|w| ..)` method takes [huffbase14::W](huffbase14::W) writer structure"]
impl crate::Writable for HUFFBASE14 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase14;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase15](huffbase15) module"]
pub type HUFFBASE15 = crate::Reg<u32, _HUFFBASE15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE15;
#[doc = "`read()` method returns [huffbase15::R](huffbase15::R) reader structure"]
impl crate::Readable for HUFFBASE15 {}
#[doc = "`write(|w| ..)` method takes [huffbase15::W](huffbase15::W) writer structure"]
impl crate::Writable for HUFFBASE15 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase15;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase16](huffbase16) module"]
pub type HUFFBASE16 = crate::Reg<u32, _HUFFBASE16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE16;
#[doc = "`read()` method returns [huffbase16::R](huffbase16::R) reader structure"]
impl crate::Readable for HUFFBASE16 {}
#[doc = "`write(|w| ..)` method takes [huffbase16::W](huffbase16::W) writer structure"]
impl crate::Writable for HUFFBASE16 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase16;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase17](huffbase17) module"]
pub type HUFFBASE17 = crate::Reg<u32, _HUFFBASE17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE17;
#[doc = "`read()` method returns [huffbase17::R](huffbase17::R) reader structure"]
impl crate::Readable for HUFFBASE17 {}
#[doc = "`write(|w| ..)` method takes [huffbase17::W](huffbase17::W) writer structure"]
impl crate::Writable for HUFFBASE17 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase17;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase18](huffbase18) module"]
pub type HUFFBASE18 = crate::Reg<u32, _HUFFBASE18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE18;
#[doc = "`read()` method returns [huffbase18::R](huffbase18::R) reader structure"]
impl crate::Readable for HUFFBASE18 {}
#[doc = "`write(|w| ..)` method takes [huffbase18::W](huffbase18::W) writer structure"]
impl crate::Writable for HUFFBASE18 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase18;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase19](huffbase19) module"]
pub type HUFFBASE19 = crate::Reg<u32, _HUFFBASE19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE19;
#[doc = "`read()` method returns [huffbase19::R](huffbase19::R) reader structure"]
impl crate::Readable for HUFFBASE19 {}
#[doc = "`write(|w| ..)` method takes [huffbase19::W](huffbase19::W) writer structure"]
impl crate::Writable for HUFFBASE19 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase19;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase20](huffbase20) module"]
pub type HUFFBASE20 = crate::Reg<u32, _HUFFBASE20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE20;
#[doc = "`read()` method returns [huffbase20::R](huffbase20::R) reader structure"]
impl crate::Readable for HUFFBASE20 {}
#[doc = "`write(|w| ..)` method takes [huffbase20::W](huffbase20::W) writer structure"]
impl crate::Writable for HUFFBASE20 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase20;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase21](huffbase21) module"]
pub type HUFFBASE21 = crate::Reg<u32, _HUFFBASE21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE21;
#[doc = "`read()` method returns [huffbase21::R](huffbase21::R) reader structure"]
impl crate::Readable for HUFFBASE21 {}
#[doc = "`write(|w| ..)` method takes [huffbase21::W](huffbase21::W) writer structure"]
impl crate::Writable for HUFFBASE21 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase21;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase22](huffbase22) module"]
pub type HUFFBASE22 = crate::Reg<u32, _HUFFBASE22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE22;
#[doc = "`read()` method returns [huffbase22::R](huffbase22::R) reader structure"]
impl crate::Readable for HUFFBASE22 {}
#[doc = "`write(|w| ..)` method takes [huffbase22::W](huffbase22::W) writer structure"]
impl crate::Writable for HUFFBASE22 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase22;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase23](huffbase23) module"]
pub type HUFFBASE23 = crate::Reg<u32, _HUFFBASE23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE23;
#[doc = "`read()` method returns [huffbase23::R](huffbase23::R) reader structure"]
impl crate::Readable for HUFFBASE23 {}
#[doc = "`write(|w| ..)` method takes [huffbase23::W](huffbase23::W) writer structure"]
impl crate::Writable for HUFFBASE23 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase23;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase24](huffbase24) module"]
pub type HUFFBASE24 = crate::Reg<u32, _HUFFBASE24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE24;
#[doc = "`read()` method returns [huffbase24::R](huffbase24::R) reader structure"]
impl crate::Readable for HUFFBASE24 {}
#[doc = "`write(|w| ..)` method takes [huffbase24::W](huffbase24::W) writer structure"]
impl crate::Writable for HUFFBASE24 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase24;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase25](huffbase25) module"]
pub type HUFFBASE25 = crate::Reg<u32, _HUFFBASE25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE25;
#[doc = "`read()` method returns [huffbase25::R](huffbase25::R) reader structure"]
impl crate::Readable for HUFFBASE25 {}
#[doc = "`write(|w| ..)` method takes [huffbase25::W](huffbase25::W) writer structure"]
impl crate::Writable for HUFFBASE25 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase25;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase26](huffbase26) module"]
pub type HUFFBASE26 = crate::Reg<u32, _HUFFBASE26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE26;
#[doc = "`read()` method returns [huffbase26::R](huffbase26::R) reader structure"]
impl crate::Readable for HUFFBASE26 {}
#[doc = "`write(|w| ..)` method takes [huffbase26::W](huffbase26::W) writer structure"]
impl crate::Writable for HUFFBASE26 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase26;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase27](huffbase27) module"]
pub type HUFFBASE27 = crate::Reg<u32, _HUFFBASE27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE27;
#[doc = "`read()` method returns [huffbase27::R](huffbase27::R) reader structure"]
impl crate::Readable for HUFFBASE27 {}
#[doc = "`write(|w| ..)` method takes [huffbase27::W](huffbase27::W) writer structure"]
impl crate::Writable for HUFFBASE27 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase27;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase28](huffbase28) module"]
pub type HUFFBASE28 = crate::Reg<u32, _HUFFBASE28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE28;
#[doc = "`read()` method returns [huffbase28::R](huffbase28::R) reader structure"]
impl crate::Readable for HUFFBASE28 {}
#[doc = "`write(|w| ..)` method takes [huffbase28::W](huffbase28::W) writer structure"]
impl crate::Writable for HUFFBASE28 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase28;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase29](huffbase29) module"]
pub type HUFFBASE29 = crate::Reg<u32, _HUFFBASE29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE29;
#[doc = "`read()` method returns [huffbase29::R](huffbase29::R) reader structure"]
impl crate::Readable for HUFFBASE29 {}
#[doc = "`write(|w| ..)` method takes [huffbase29::W](huffbase29::W) writer structure"]
impl crate::Writable for HUFFBASE29 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase29;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase30](huffbase30) module"]
pub type HUFFBASE30 = crate::Reg<u32, _HUFFBASE30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE30;
#[doc = "`read()` method returns [huffbase30::R](huffbase30::R) reader structure"]
impl crate::Readable for HUFFBASE30 {}
#[doc = "`write(|w| ..)` method takes [huffbase30::W](huffbase30::W) writer structure"]
impl crate::Writable for HUFFBASE30 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase30;
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase31](huffbase31) module"]
pub type HUFFBASE31 = crate::Reg<u32, _HUFFBASE31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFBASE31;
#[doc = "`read()` method returns [huffbase31::R](huffbase31::R) reader structure"]
impl crate::Readable for HUFFBASE31 {}
#[doc = "`write(|w| ..)` method takes [huffbase31::W](huffbase31::W) writer structure"]
impl crate::Writable for HUFFBASE31 {}
#[doc = "JPEG HuffSymb tables"]
pub mod huffbase31;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb0](huffsymb0) module"]
pub type HUFFSYMB0 = crate::Reg<u32, _HUFFSYMB0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB0;
#[doc = "`read()` method returns [huffsymb0::R](huffsymb0::R) reader structure"]
impl crate::Readable for HUFFSYMB0 {}
#[doc = "`write(|w| ..)` method takes [huffsymb0::W](huffsymb0::W) writer structure"]
impl crate::Writable for HUFFSYMB0 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb0;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb1](huffsymb1) module"]
pub type HUFFSYMB1 = crate::Reg<u32, _HUFFSYMB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB1;
#[doc = "`read()` method returns [huffsymb1::R](huffsymb1::R) reader structure"]
impl crate::Readable for HUFFSYMB1 {}
#[doc = "`write(|w| ..)` method takes [huffsymb1::W](huffsymb1::W) writer structure"]
impl crate::Writable for HUFFSYMB1 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb1;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb2](huffsymb2) module"]
pub type HUFFSYMB2 = crate::Reg<u32, _HUFFSYMB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB2;
#[doc = "`read()` method returns [huffsymb2::R](huffsymb2::R) reader structure"]
impl crate::Readable for HUFFSYMB2 {}
#[doc = "`write(|w| ..)` method takes [huffsymb2::W](huffsymb2::W) writer structure"]
impl crate::Writable for HUFFSYMB2 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb2;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb3](huffsymb3) module"]
pub type HUFFSYMB3 = crate::Reg<u32, _HUFFSYMB3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB3;
#[doc = "`read()` method returns [huffsymb3::R](huffsymb3::R) reader structure"]
impl crate::Readable for HUFFSYMB3 {}
#[doc = "`write(|w| ..)` method takes [huffsymb3::W](huffsymb3::W) writer structure"]
impl crate::Writable for HUFFSYMB3 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb3;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb4](huffsymb4) module"]
pub type HUFFSYMB4 = crate::Reg<u32, _HUFFSYMB4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB4;
#[doc = "`read()` method returns [huffsymb4::R](huffsymb4::R) reader structure"]
impl crate::Readable for HUFFSYMB4 {}
#[doc = "`write(|w| ..)` method takes [huffsymb4::W](huffsymb4::W) writer structure"]
impl crate::Writable for HUFFSYMB4 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb4;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb5](huffsymb5) module"]
pub type HUFFSYMB5 = crate::Reg<u32, _HUFFSYMB5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB5;
#[doc = "`read()` method returns [huffsymb5::R](huffsymb5::R) reader structure"]
impl crate::Readable for HUFFSYMB5 {}
#[doc = "`write(|w| ..)` method takes [huffsymb5::W](huffsymb5::W) writer structure"]
impl crate::Writable for HUFFSYMB5 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb5;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb6](huffsymb6) module"]
pub type HUFFSYMB6 = crate::Reg<u32, _HUFFSYMB6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB6;
#[doc = "`read()` method returns [huffsymb6::R](huffsymb6::R) reader structure"]
impl crate::Readable for HUFFSYMB6 {}
#[doc = "`write(|w| ..)` method takes [huffsymb6::W](huffsymb6::W) writer structure"]
impl crate::Writable for HUFFSYMB6 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb6;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb7](huffsymb7) module"]
pub type HUFFSYMB7 = crate::Reg<u32, _HUFFSYMB7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB7;
#[doc = "`read()` method returns [huffsymb7::R](huffsymb7::R) reader structure"]
impl crate::Readable for HUFFSYMB7 {}
#[doc = "`write(|w| ..)` method takes [huffsymb7::W](huffsymb7::W) writer structure"]
impl crate::Writable for HUFFSYMB7 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb7;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb8](huffsymb8) module"]
pub type HUFFSYMB8 = crate::Reg<u32, _HUFFSYMB8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB8;
#[doc = "`read()` method returns [huffsymb8::R](huffsymb8::R) reader structure"]
impl crate::Readable for HUFFSYMB8 {}
#[doc = "`write(|w| ..)` method takes [huffsymb8::W](huffsymb8::W) writer structure"]
impl crate::Writable for HUFFSYMB8 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb8;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb9](huffsymb9) module"]
pub type HUFFSYMB9 = crate::Reg<u32, _HUFFSYMB9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB9;
#[doc = "`read()` method returns [huffsymb9::R](huffsymb9::R) reader structure"]
impl crate::Readable for HUFFSYMB9 {}
#[doc = "`write(|w| ..)` method takes [huffsymb9::W](huffsymb9::W) writer structure"]
impl crate::Writable for HUFFSYMB9 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb9;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb10](huffsymb10) module"]
pub type HUFFSYMB10 = crate::Reg<u32, _HUFFSYMB10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB10;
#[doc = "`read()` method returns [huffsymb10::R](huffsymb10::R) reader structure"]
impl crate::Readable for HUFFSYMB10 {}
#[doc = "`write(|w| ..)` method takes [huffsymb10::W](huffsymb10::W) writer structure"]
impl crate::Writable for HUFFSYMB10 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb10;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb11](huffsymb11) module"]
pub type HUFFSYMB11 = crate::Reg<u32, _HUFFSYMB11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB11;
#[doc = "`read()` method returns [huffsymb11::R](huffsymb11::R) reader structure"]
impl crate::Readable for HUFFSYMB11 {}
#[doc = "`write(|w| ..)` method takes [huffsymb11::W](huffsymb11::W) writer structure"]
impl crate::Writable for HUFFSYMB11 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb11;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb12](huffsymb12) module"]
pub type HUFFSYMB12 = crate::Reg<u32, _HUFFSYMB12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB12;
#[doc = "`read()` method returns [huffsymb12::R](huffsymb12::R) reader structure"]
impl crate::Readable for HUFFSYMB12 {}
#[doc = "`write(|w| ..)` method takes [huffsymb12::W](huffsymb12::W) writer structure"]
impl crate::Writable for HUFFSYMB12 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb12;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb13](huffsymb13) module"]
pub type HUFFSYMB13 = crate::Reg<u32, _HUFFSYMB13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB13;
#[doc = "`read()` method returns [huffsymb13::R](huffsymb13::R) reader structure"]
impl crate::Readable for HUFFSYMB13 {}
#[doc = "`write(|w| ..)` method takes [huffsymb13::W](huffsymb13::W) writer structure"]
impl crate::Writable for HUFFSYMB13 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb13;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb14](huffsymb14) module"]
pub type HUFFSYMB14 = crate::Reg<u32, _HUFFSYMB14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB14;
#[doc = "`read()` method returns [huffsymb14::R](huffsymb14::R) reader structure"]
impl crate::Readable for HUFFSYMB14 {}
#[doc = "`write(|w| ..)` method takes [huffsymb14::W](huffsymb14::W) writer structure"]
impl crate::Writable for HUFFSYMB14 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb14;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb15](huffsymb15) module"]
pub type HUFFSYMB15 = crate::Reg<u32, _HUFFSYMB15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB15;
#[doc = "`read()` method returns [huffsymb15::R](huffsymb15::R) reader structure"]
impl crate::Readable for HUFFSYMB15 {}
#[doc = "`write(|w| ..)` method takes [huffsymb15::W](huffsymb15::W) writer structure"]
impl crate::Writable for HUFFSYMB15 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb15;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb16](huffsymb16) module"]
pub type HUFFSYMB16 = crate::Reg<u32, _HUFFSYMB16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB16;
#[doc = "`read()` method returns [huffsymb16::R](huffsymb16::R) reader structure"]
impl crate::Readable for HUFFSYMB16 {}
#[doc = "`write(|w| ..)` method takes [huffsymb16::W](huffsymb16::W) writer structure"]
impl crate::Writable for HUFFSYMB16 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb16;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb17](huffsymb17) module"]
pub type HUFFSYMB17 = crate::Reg<u32, _HUFFSYMB17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB17;
#[doc = "`read()` method returns [huffsymb17::R](huffsymb17::R) reader structure"]
impl crate::Readable for HUFFSYMB17 {}
#[doc = "`write(|w| ..)` method takes [huffsymb17::W](huffsymb17::W) writer structure"]
impl crate::Writable for HUFFSYMB17 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb17;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb18](huffsymb18) module"]
pub type HUFFSYMB18 = crate::Reg<u32, _HUFFSYMB18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB18;
#[doc = "`read()` method returns [huffsymb18::R](huffsymb18::R) reader structure"]
impl crate::Readable for HUFFSYMB18 {}
#[doc = "`write(|w| ..)` method takes [huffsymb18::W](huffsymb18::W) writer structure"]
impl crate::Writable for HUFFSYMB18 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb18;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb19](huffsymb19) module"]
pub type HUFFSYMB19 = crate::Reg<u32, _HUFFSYMB19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB19;
#[doc = "`read()` method returns [huffsymb19::R](huffsymb19::R) reader structure"]
impl crate::Readable for HUFFSYMB19 {}
#[doc = "`write(|w| ..)` method takes [huffsymb19::W](huffsymb19::W) writer structure"]
impl crate::Writable for HUFFSYMB19 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb19;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb20](huffsymb20) module"]
pub type HUFFSYMB20 = crate::Reg<u32, _HUFFSYMB20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB20;
#[doc = "`read()` method returns [huffsymb20::R](huffsymb20::R) reader structure"]
impl crate::Readable for HUFFSYMB20 {}
#[doc = "`write(|w| ..)` method takes [huffsymb20::W](huffsymb20::W) writer structure"]
impl crate::Writable for HUFFSYMB20 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb20;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb21](huffsymb21) module"]
pub type HUFFSYMB21 = crate::Reg<u32, _HUFFSYMB21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB21;
#[doc = "`read()` method returns [huffsymb21::R](huffsymb21::R) reader structure"]
impl crate::Readable for HUFFSYMB21 {}
#[doc = "`write(|w| ..)` method takes [huffsymb21::W](huffsymb21::W) writer structure"]
impl crate::Writable for HUFFSYMB21 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb21;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb22](huffsymb22) module"]
pub type HUFFSYMB22 = crate::Reg<u32, _HUFFSYMB22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB22;
#[doc = "`read()` method returns [huffsymb22::R](huffsymb22::R) reader structure"]
impl crate::Readable for HUFFSYMB22 {}
#[doc = "`write(|w| ..)` method takes [huffsymb22::W](huffsymb22::W) writer structure"]
impl crate::Writable for HUFFSYMB22 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb22;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb23](huffsymb23) module"]
pub type HUFFSYMB23 = crate::Reg<u32, _HUFFSYMB23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB23;
#[doc = "`read()` method returns [huffsymb23::R](huffsymb23::R) reader structure"]
impl crate::Readable for HUFFSYMB23 {}
#[doc = "`write(|w| ..)` method takes [huffsymb23::W](huffsymb23::W) writer structure"]
impl crate::Writable for HUFFSYMB23 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb23;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb24](huffsymb24) module"]
pub type HUFFSYMB24 = crate::Reg<u32, _HUFFSYMB24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB24;
#[doc = "`read()` method returns [huffsymb24::R](huffsymb24::R) reader structure"]
impl crate::Readable for HUFFSYMB24 {}
#[doc = "`write(|w| ..)` method takes [huffsymb24::W](huffsymb24::W) writer structure"]
impl crate::Writable for HUFFSYMB24 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb24;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb25](huffsymb25) module"]
pub type HUFFSYMB25 = crate::Reg<u32, _HUFFSYMB25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB25;
#[doc = "`read()` method returns [huffsymb25::R](huffsymb25::R) reader structure"]
impl crate::Readable for HUFFSYMB25 {}
#[doc = "`write(|w| ..)` method takes [huffsymb25::W](huffsymb25::W) writer structure"]
impl crate::Writable for HUFFSYMB25 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb25;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb26](huffsymb26) module"]
pub type HUFFSYMB26 = crate::Reg<u32, _HUFFSYMB26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB26;
#[doc = "`read()` method returns [huffsymb26::R](huffsymb26::R) reader structure"]
impl crate::Readable for HUFFSYMB26 {}
#[doc = "`write(|w| ..)` method takes [huffsymb26::W](huffsymb26::W) writer structure"]
impl crate::Writable for HUFFSYMB26 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb26;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb27](huffsymb27) module"]
pub type HUFFSYMB27 = crate::Reg<u32, _HUFFSYMB27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB27;
#[doc = "`read()` method returns [huffsymb27::R](huffsymb27::R) reader structure"]
impl crate::Readable for HUFFSYMB27 {}
#[doc = "`write(|w| ..)` method takes [huffsymb27::W](huffsymb27::W) writer structure"]
impl crate::Writable for HUFFSYMB27 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb27;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb28](huffsymb28) module"]
pub type HUFFSYMB28 = crate::Reg<u32, _HUFFSYMB28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB28;
#[doc = "`read()` method returns [huffsymb28::R](huffsymb28::R) reader structure"]
impl crate::Readable for HUFFSYMB28 {}
#[doc = "`write(|w| ..)` method takes [huffsymb28::W](huffsymb28::W) writer structure"]
impl crate::Writable for HUFFSYMB28 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb28;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb29](huffsymb29) module"]
pub type HUFFSYMB29 = crate::Reg<u32, _HUFFSYMB29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB29;
#[doc = "`read()` method returns [huffsymb29::R](huffsymb29::R) reader structure"]
impl crate::Readable for HUFFSYMB29 {}
#[doc = "`write(|w| ..)` method takes [huffsymb29::W](huffsymb29::W) writer structure"]
impl crate::Writable for HUFFSYMB29 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb29;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb30](huffsymb30) module"]
pub type HUFFSYMB30 = crate::Reg<u32, _HUFFSYMB30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB30;
#[doc = "`read()` method returns [huffsymb30::R](huffsymb30::R) reader structure"]
impl crate::Readable for HUFFSYMB30 {}
#[doc = "`write(|w| ..)` method takes [huffsymb30::W](huffsymb30::W) writer structure"]
impl crate::Writable for HUFFSYMB30 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb30;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb31](huffsymb31) module"]
pub type HUFFSYMB31 = crate::Reg<u32, _HUFFSYMB31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB31;
#[doc = "`read()` method returns [huffsymb31::R](huffsymb31::R) reader structure"]
impl crate::Readable for HUFFSYMB31 {}
#[doc = "`write(|w| ..)` method takes [huffsymb31::W](huffsymb31::W) writer structure"]
impl crate::Writable for HUFFSYMB31 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb31;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb32](huffsymb32) module"]
pub type HUFFSYMB32 = crate::Reg<u32, _HUFFSYMB32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB32;
#[doc = "`read()` method returns [huffsymb32::R](huffsymb32::R) reader structure"]
impl crate::Readable for HUFFSYMB32 {}
#[doc = "`write(|w| ..)` method takes [huffsymb32::W](huffsymb32::W) writer structure"]
impl crate::Writable for HUFFSYMB32 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb32;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb33](huffsymb33) module"]
pub type HUFFSYMB33 = crate::Reg<u32, _HUFFSYMB33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB33;
#[doc = "`read()` method returns [huffsymb33::R](huffsymb33::R) reader structure"]
impl crate::Readable for HUFFSYMB33 {}
#[doc = "`write(|w| ..)` method takes [huffsymb33::W](huffsymb33::W) writer structure"]
impl crate::Writable for HUFFSYMB33 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb33;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb34](huffsymb34) module"]
pub type HUFFSYMB34 = crate::Reg<u32, _HUFFSYMB34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB34;
#[doc = "`read()` method returns [huffsymb34::R](huffsymb34::R) reader structure"]
impl crate::Readable for HUFFSYMB34 {}
#[doc = "`write(|w| ..)` method takes [huffsymb34::W](huffsymb34::W) writer structure"]
impl crate::Writable for HUFFSYMB34 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb34;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb35](huffsymb35) module"]
pub type HUFFSYMB35 = crate::Reg<u32, _HUFFSYMB35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB35;
#[doc = "`read()` method returns [huffsymb35::R](huffsymb35::R) reader structure"]
impl crate::Readable for HUFFSYMB35 {}
#[doc = "`write(|w| ..)` method takes [huffsymb35::W](huffsymb35::W) writer structure"]
impl crate::Writable for HUFFSYMB35 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb35;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb36](huffsymb36) module"]
pub type HUFFSYMB36 = crate::Reg<u32, _HUFFSYMB36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB36;
#[doc = "`read()` method returns [huffsymb36::R](huffsymb36::R) reader structure"]
impl crate::Readable for HUFFSYMB36 {}
#[doc = "`write(|w| ..)` method takes [huffsymb36::W](huffsymb36::W) writer structure"]
impl crate::Writable for HUFFSYMB36 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb36;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb37](huffsymb37) module"]
pub type HUFFSYMB37 = crate::Reg<u32, _HUFFSYMB37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB37;
#[doc = "`read()` method returns [huffsymb37::R](huffsymb37::R) reader structure"]
impl crate::Readable for HUFFSYMB37 {}
#[doc = "`write(|w| ..)` method takes [huffsymb37::W](huffsymb37::W) writer structure"]
impl crate::Writable for HUFFSYMB37 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb37;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb38](huffsymb38) module"]
pub type HUFFSYMB38 = crate::Reg<u32, _HUFFSYMB38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB38;
#[doc = "`read()` method returns [huffsymb38::R](huffsymb38::R) reader structure"]
impl crate::Readable for HUFFSYMB38 {}
#[doc = "`write(|w| ..)` method takes [huffsymb38::W](huffsymb38::W) writer structure"]
impl crate::Writable for HUFFSYMB38 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb38;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb39](huffsymb39) module"]
pub type HUFFSYMB39 = crate::Reg<u32, _HUFFSYMB39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB39;
#[doc = "`read()` method returns [huffsymb39::R](huffsymb39::R) reader structure"]
impl crate::Readable for HUFFSYMB39 {}
#[doc = "`write(|w| ..)` method takes [huffsymb39::W](huffsymb39::W) writer structure"]
impl crate::Writable for HUFFSYMB39 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb39;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb40](huffsymb40) module"]
pub type HUFFSYMB40 = crate::Reg<u32, _HUFFSYMB40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB40;
#[doc = "`read()` method returns [huffsymb40::R](huffsymb40::R) reader structure"]
impl crate::Readable for HUFFSYMB40 {}
#[doc = "`write(|w| ..)` method takes [huffsymb40::W](huffsymb40::W) writer structure"]
impl crate::Writable for HUFFSYMB40 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb40;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb41](huffsymb41) module"]
pub type HUFFSYMB41 = crate::Reg<u32, _HUFFSYMB41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB41;
#[doc = "`read()` method returns [huffsymb41::R](huffsymb41::R) reader structure"]
impl crate::Readable for HUFFSYMB41 {}
#[doc = "`write(|w| ..)` method takes [huffsymb41::W](huffsymb41::W) writer structure"]
impl crate::Writable for HUFFSYMB41 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb41;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb42](huffsymb42) module"]
pub type HUFFSYMB42 = crate::Reg<u32, _HUFFSYMB42>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB42;
#[doc = "`read()` method returns [huffsymb42::R](huffsymb42::R) reader structure"]
impl crate::Readable for HUFFSYMB42 {}
#[doc = "`write(|w| ..)` method takes [huffsymb42::W](huffsymb42::W) writer structure"]
impl crate::Writable for HUFFSYMB42 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb42;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb43](huffsymb43) module"]
pub type HUFFSYMB43 = crate::Reg<u32, _HUFFSYMB43>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB43;
#[doc = "`read()` method returns [huffsymb43::R](huffsymb43::R) reader structure"]
impl crate::Readable for HUFFSYMB43 {}
#[doc = "`write(|w| ..)` method takes [huffsymb43::W](huffsymb43::W) writer structure"]
impl crate::Writable for HUFFSYMB43 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb43;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb44](huffsymb44) module"]
pub type HUFFSYMB44 = crate::Reg<u32, _HUFFSYMB44>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB44;
#[doc = "`read()` method returns [huffsymb44::R](huffsymb44::R) reader structure"]
impl crate::Readable for HUFFSYMB44 {}
#[doc = "`write(|w| ..)` method takes [huffsymb44::W](huffsymb44::W) writer structure"]
impl crate::Writable for HUFFSYMB44 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb44;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb45](huffsymb45) module"]
pub type HUFFSYMB45 = crate::Reg<u32, _HUFFSYMB45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB45;
#[doc = "`read()` method returns [huffsymb45::R](huffsymb45::R) reader structure"]
impl crate::Readable for HUFFSYMB45 {}
#[doc = "`write(|w| ..)` method takes [huffsymb45::W](huffsymb45::W) writer structure"]
impl crate::Writable for HUFFSYMB45 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb45;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb46](huffsymb46) module"]
pub type HUFFSYMB46 = crate::Reg<u32, _HUFFSYMB46>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB46;
#[doc = "`read()` method returns [huffsymb46::R](huffsymb46::R) reader structure"]
impl crate::Readable for HUFFSYMB46 {}
#[doc = "`write(|w| ..)` method takes [huffsymb46::W](huffsymb46::W) writer structure"]
impl crate::Writable for HUFFSYMB46 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb46;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb47](huffsymb47) module"]
pub type HUFFSYMB47 = crate::Reg<u32, _HUFFSYMB47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB47;
#[doc = "`read()` method returns [huffsymb47::R](huffsymb47::R) reader structure"]
impl crate::Readable for HUFFSYMB47 {}
#[doc = "`write(|w| ..)` method takes [huffsymb47::W](huffsymb47::W) writer structure"]
impl crate::Writable for HUFFSYMB47 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb47;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb48](huffsymb48) module"]
pub type HUFFSYMB48 = crate::Reg<u32, _HUFFSYMB48>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB48;
#[doc = "`read()` method returns [huffsymb48::R](huffsymb48::R) reader structure"]
impl crate::Readable for HUFFSYMB48 {}
#[doc = "`write(|w| ..)` method takes [huffsymb48::W](huffsymb48::W) writer structure"]
impl crate::Writable for HUFFSYMB48 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb48;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb49](huffsymb49) module"]
pub type HUFFSYMB49 = crate::Reg<u32, _HUFFSYMB49>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB49;
#[doc = "`read()` method returns [huffsymb49::R](huffsymb49::R) reader structure"]
impl crate::Readable for HUFFSYMB49 {}
#[doc = "`write(|w| ..)` method takes [huffsymb49::W](huffsymb49::W) writer structure"]
impl crate::Writable for HUFFSYMB49 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb49;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb50](huffsymb50) module"]
pub type HUFFSYMB50 = crate::Reg<u32, _HUFFSYMB50>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB50;
#[doc = "`read()` method returns [huffsymb50::R](huffsymb50::R) reader structure"]
impl crate::Readable for HUFFSYMB50 {}
#[doc = "`write(|w| ..)` method takes [huffsymb50::W](huffsymb50::W) writer structure"]
impl crate::Writable for HUFFSYMB50 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb50;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb51](huffsymb51) module"]
pub type HUFFSYMB51 = crate::Reg<u32, _HUFFSYMB51>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB51;
#[doc = "`read()` method returns [huffsymb51::R](huffsymb51::R) reader structure"]
impl crate::Readable for HUFFSYMB51 {}
#[doc = "`write(|w| ..)` method takes [huffsymb51::W](huffsymb51::W) writer structure"]
impl crate::Writable for HUFFSYMB51 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb51;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb52](huffsymb52) module"]
pub type HUFFSYMB52 = crate::Reg<u32, _HUFFSYMB52>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB52;
#[doc = "`read()` method returns [huffsymb52::R](huffsymb52::R) reader structure"]
impl crate::Readable for HUFFSYMB52 {}
#[doc = "`write(|w| ..)` method takes [huffsymb52::W](huffsymb52::W) writer structure"]
impl crate::Writable for HUFFSYMB52 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb52;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb53](huffsymb53) module"]
pub type HUFFSYMB53 = crate::Reg<u32, _HUFFSYMB53>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB53;
#[doc = "`read()` method returns [huffsymb53::R](huffsymb53::R) reader structure"]
impl crate::Readable for HUFFSYMB53 {}
#[doc = "`write(|w| ..)` method takes [huffsymb53::W](huffsymb53::W) writer structure"]
impl crate::Writable for HUFFSYMB53 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb53;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb54](huffsymb54) module"]
pub type HUFFSYMB54 = crate::Reg<u32, _HUFFSYMB54>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB54;
#[doc = "`read()` method returns [huffsymb54::R](huffsymb54::R) reader structure"]
impl crate::Readable for HUFFSYMB54 {}
#[doc = "`write(|w| ..)` method takes [huffsymb54::W](huffsymb54::W) writer structure"]
impl crate::Writable for HUFFSYMB54 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb54;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb55](huffsymb55) module"]
pub type HUFFSYMB55 = crate::Reg<u32, _HUFFSYMB55>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB55;
#[doc = "`read()` method returns [huffsymb55::R](huffsymb55::R) reader structure"]
impl crate::Readable for HUFFSYMB55 {}
#[doc = "`write(|w| ..)` method takes [huffsymb55::W](huffsymb55::W) writer structure"]
impl crate::Writable for HUFFSYMB55 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb55;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb56](huffsymb56) module"]
pub type HUFFSYMB56 = crate::Reg<u32, _HUFFSYMB56>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB56;
#[doc = "`read()` method returns [huffsymb56::R](huffsymb56::R) reader structure"]
impl crate::Readable for HUFFSYMB56 {}
#[doc = "`write(|w| ..)` method takes [huffsymb56::W](huffsymb56::W) writer structure"]
impl crate::Writable for HUFFSYMB56 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb56;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb57](huffsymb57) module"]
pub type HUFFSYMB57 = crate::Reg<u32, _HUFFSYMB57>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB57;
#[doc = "`read()` method returns [huffsymb57::R](huffsymb57::R) reader structure"]
impl crate::Readable for HUFFSYMB57 {}
#[doc = "`write(|w| ..)` method takes [huffsymb57::W](huffsymb57::W) writer structure"]
impl crate::Writable for HUFFSYMB57 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb57;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb58](huffsymb58) module"]
pub type HUFFSYMB58 = crate::Reg<u32, _HUFFSYMB58>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB58;
#[doc = "`read()` method returns [huffsymb58::R](huffsymb58::R) reader structure"]
impl crate::Readable for HUFFSYMB58 {}
#[doc = "`write(|w| ..)` method takes [huffsymb58::W](huffsymb58::W) writer structure"]
impl crate::Writable for HUFFSYMB58 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb58;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb59](huffsymb59) module"]
pub type HUFFSYMB59 = crate::Reg<u32, _HUFFSYMB59>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB59;
#[doc = "`read()` method returns [huffsymb59::R](huffsymb59::R) reader structure"]
impl crate::Readable for HUFFSYMB59 {}
#[doc = "`write(|w| ..)` method takes [huffsymb59::W](huffsymb59::W) writer structure"]
impl crate::Writable for HUFFSYMB59 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb59;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb60](huffsymb60) module"]
pub type HUFFSYMB60 = crate::Reg<u32, _HUFFSYMB60>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB60;
#[doc = "`read()` method returns [huffsymb60::R](huffsymb60::R) reader structure"]
impl crate::Readable for HUFFSYMB60 {}
#[doc = "`write(|w| ..)` method takes [huffsymb60::W](huffsymb60::W) writer structure"]
impl crate::Writable for HUFFSYMB60 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb60;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb61](huffsymb61) module"]
pub type HUFFSYMB61 = crate::Reg<u32, _HUFFSYMB61>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB61;
#[doc = "`read()` method returns [huffsymb61::R](huffsymb61::R) reader structure"]
impl crate::Readable for HUFFSYMB61 {}
#[doc = "`write(|w| ..)` method takes [huffsymb61::W](huffsymb61::W) writer structure"]
impl crate::Writable for HUFFSYMB61 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb61;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb62](huffsymb62) module"]
pub type HUFFSYMB62 = crate::Reg<u32, _HUFFSYMB62>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB62;
#[doc = "`read()` method returns [huffsymb62::R](huffsymb62::R) reader structure"]
impl crate::Readable for HUFFSYMB62 {}
#[doc = "`write(|w| ..)` method takes [huffsymb62::W](huffsymb62::W) writer structure"]
impl crate::Writable for HUFFSYMB62 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb62;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb63](huffsymb63) module"]
pub type HUFFSYMB63 = crate::Reg<u32, _HUFFSYMB63>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB63;
#[doc = "`read()` method returns [huffsymb63::R](huffsymb63::R) reader structure"]
impl crate::Readable for HUFFSYMB63 {}
#[doc = "`write(|w| ..)` method takes [huffsymb63::W](huffsymb63::W) writer structure"]
impl crate::Writable for HUFFSYMB63 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb63;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb64](huffsymb64) module"]
pub type HUFFSYMB64 = crate::Reg<u32, _HUFFSYMB64>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB64;
#[doc = "`read()` method returns [huffsymb64::R](huffsymb64::R) reader structure"]
impl crate::Readable for HUFFSYMB64 {}
#[doc = "`write(|w| ..)` method takes [huffsymb64::W](huffsymb64::W) writer structure"]
impl crate::Writable for HUFFSYMB64 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb64;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb65](huffsymb65) module"]
pub type HUFFSYMB65 = crate::Reg<u32, _HUFFSYMB65>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB65;
#[doc = "`read()` method returns [huffsymb65::R](huffsymb65::R) reader structure"]
impl crate::Readable for HUFFSYMB65 {}
#[doc = "`write(|w| ..)` method takes [huffsymb65::W](huffsymb65::W) writer structure"]
impl crate::Writable for HUFFSYMB65 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb65;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb66](huffsymb66) module"]
pub type HUFFSYMB66 = crate::Reg<u32, _HUFFSYMB66>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB66;
#[doc = "`read()` method returns [huffsymb66::R](huffsymb66::R) reader structure"]
impl crate::Readable for HUFFSYMB66 {}
#[doc = "`write(|w| ..)` method takes [huffsymb66::W](huffsymb66::W) writer structure"]
impl crate::Writable for HUFFSYMB66 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb66;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb67](huffsymb67) module"]
pub type HUFFSYMB67 = crate::Reg<u32, _HUFFSYMB67>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB67;
#[doc = "`read()` method returns [huffsymb67::R](huffsymb67::R) reader structure"]
impl crate::Readable for HUFFSYMB67 {}
#[doc = "`write(|w| ..)` method takes [huffsymb67::W](huffsymb67::W) writer structure"]
impl crate::Writable for HUFFSYMB67 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb67;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb68](huffsymb68) module"]
pub type HUFFSYMB68 = crate::Reg<u32, _HUFFSYMB68>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB68;
#[doc = "`read()` method returns [huffsymb68::R](huffsymb68::R) reader structure"]
impl crate::Readable for HUFFSYMB68 {}
#[doc = "`write(|w| ..)` method takes [huffsymb68::W](huffsymb68::W) writer structure"]
impl crate::Writable for HUFFSYMB68 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb68;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb69](huffsymb69) module"]
pub type HUFFSYMB69 = crate::Reg<u32, _HUFFSYMB69>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB69;
#[doc = "`read()` method returns [huffsymb69::R](huffsymb69::R) reader structure"]
impl crate::Readable for HUFFSYMB69 {}
#[doc = "`write(|w| ..)` method takes [huffsymb69::W](huffsymb69::W) writer structure"]
impl crate::Writable for HUFFSYMB69 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb69;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb70](huffsymb70) module"]
pub type HUFFSYMB70 = crate::Reg<u32, _HUFFSYMB70>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB70;
#[doc = "`read()` method returns [huffsymb70::R](huffsymb70::R) reader structure"]
impl crate::Readable for HUFFSYMB70 {}
#[doc = "`write(|w| ..)` method takes [huffsymb70::W](huffsymb70::W) writer structure"]
impl crate::Writable for HUFFSYMB70 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb70;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb71](huffsymb71) module"]
pub type HUFFSYMB71 = crate::Reg<u32, _HUFFSYMB71>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB71;
#[doc = "`read()` method returns [huffsymb71::R](huffsymb71::R) reader structure"]
impl crate::Readable for HUFFSYMB71 {}
#[doc = "`write(|w| ..)` method takes [huffsymb71::W](huffsymb71::W) writer structure"]
impl crate::Writable for HUFFSYMB71 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb71;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb72](huffsymb72) module"]
pub type HUFFSYMB72 = crate::Reg<u32, _HUFFSYMB72>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB72;
#[doc = "`read()` method returns [huffsymb72::R](huffsymb72::R) reader structure"]
impl crate::Readable for HUFFSYMB72 {}
#[doc = "`write(|w| ..)` method takes [huffsymb72::W](huffsymb72::W) writer structure"]
impl crate::Writable for HUFFSYMB72 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb72;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb73](huffsymb73) module"]
pub type HUFFSYMB73 = crate::Reg<u32, _HUFFSYMB73>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB73;
#[doc = "`read()` method returns [huffsymb73::R](huffsymb73::R) reader structure"]
impl crate::Readable for HUFFSYMB73 {}
#[doc = "`write(|w| ..)` method takes [huffsymb73::W](huffsymb73::W) writer structure"]
impl crate::Writable for HUFFSYMB73 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb73;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb74](huffsymb74) module"]
pub type HUFFSYMB74 = crate::Reg<u32, _HUFFSYMB74>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB74;
#[doc = "`read()` method returns [huffsymb74::R](huffsymb74::R) reader structure"]
impl crate::Readable for HUFFSYMB74 {}
#[doc = "`write(|w| ..)` method takes [huffsymb74::W](huffsymb74::W) writer structure"]
impl crate::Writable for HUFFSYMB74 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb74;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb75](huffsymb75) module"]
pub type HUFFSYMB75 = crate::Reg<u32, _HUFFSYMB75>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB75;
#[doc = "`read()` method returns [huffsymb75::R](huffsymb75::R) reader structure"]
impl crate::Readable for HUFFSYMB75 {}
#[doc = "`write(|w| ..)` method takes [huffsymb75::W](huffsymb75::W) writer structure"]
impl crate::Writable for HUFFSYMB75 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb75;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb76](huffsymb76) module"]
pub type HUFFSYMB76 = crate::Reg<u32, _HUFFSYMB76>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB76;
#[doc = "`read()` method returns [huffsymb76::R](huffsymb76::R) reader structure"]
impl crate::Readable for HUFFSYMB76 {}
#[doc = "`write(|w| ..)` method takes [huffsymb76::W](huffsymb76::W) writer structure"]
impl crate::Writable for HUFFSYMB76 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb76;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb77](huffsymb77) module"]
pub type HUFFSYMB77 = crate::Reg<u32, _HUFFSYMB77>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB77;
#[doc = "`read()` method returns [huffsymb77::R](huffsymb77::R) reader structure"]
impl crate::Readable for HUFFSYMB77 {}
#[doc = "`write(|w| ..)` method takes [huffsymb77::W](huffsymb77::W) writer structure"]
impl crate::Writable for HUFFSYMB77 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb77;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb78](huffsymb78) module"]
pub type HUFFSYMB78 = crate::Reg<u32, _HUFFSYMB78>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB78;
#[doc = "`read()` method returns [huffsymb78::R](huffsymb78::R) reader structure"]
impl crate::Readable for HUFFSYMB78 {}
#[doc = "`write(|w| ..)` method takes [huffsymb78::W](huffsymb78::W) writer structure"]
impl crate::Writable for HUFFSYMB78 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb78;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb79](huffsymb79) module"]
pub type HUFFSYMB79 = crate::Reg<u32, _HUFFSYMB79>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB79;
#[doc = "`read()` method returns [huffsymb79::R](huffsymb79::R) reader structure"]
impl crate::Readable for HUFFSYMB79 {}
#[doc = "`write(|w| ..)` method takes [huffsymb79::W](huffsymb79::W) writer structure"]
impl crate::Writable for HUFFSYMB79 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb79;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb80](huffsymb80) module"]
pub type HUFFSYMB80 = crate::Reg<u32, _HUFFSYMB80>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB80;
#[doc = "`read()` method returns [huffsymb80::R](huffsymb80::R) reader structure"]
impl crate::Readable for HUFFSYMB80 {}
#[doc = "`write(|w| ..)` method takes [huffsymb80::W](huffsymb80::W) writer structure"]
impl crate::Writable for HUFFSYMB80 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb80;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb81](huffsymb81) module"]
pub type HUFFSYMB81 = crate::Reg<u32, _HUFFSYMB81>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB81;
#[doc = "`read()` method returns [huffsymb81::R](huffsymb81::R) reader structure"]
impl crate::Readable for HUFFSYMB81 {}
#[doc = "`write(|w| ..)` method takes [huffsymb81::W](huffsymb81::W) writer structure"]
impl crate::Writable for HUFFSYMB81 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb81;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb82](huffsymb82) module"]
pub type HUFFSYMB82 = crate::Reg<u32, _HUFFSYMB82>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB82;
#[doc = "`read()` method returns [huffsymb82::R](huffsymb82::R) reader structure"]
impl crate::Readable for HUFFSYMB82 {}
#[doc = "`write(|w| ..)` method takes [huffsymb82::W](huffsymb82::W) writer structure"]
impl crate::Writable for HUFFSYMB82 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb82;
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb83](huffsymb83) module"]
pub type HUFFSYMB83 = crate::Reg<u32, _HUFFSYMB83>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFSYMB83;
#[doc = "`read()` method returns [huffsymb83::R](huffsymb83::R) reader structure"]
impl crate::Readable for HUFFSYMB83 {}
#[doc = "`write(|w| ..)` method takes [huffsymb83::W](huffsymb83::W) writer structure"]
impl crate::Writable for HUFFSYMB83 {}
#[doc = "JPEG HUFFSYMB tables"]
pub mod huffsymb83;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem0](dhtmem0) module"]
pub type DHTMEM0 = crate::Reg<u32, _DHTMEM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM0;
#[doc = "`read()` method returns [dhtmem0::R](dhtmem0::R) reader structure"]
impl crate::Readable for DHTMEM0 {}
#[doc = "`write(|w| ..)` method takes [dhtmem0::W](dhtmem0::W) writer structure"]
impl crate::Writable for DHTMEM0 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem0;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem2](dhtmem2) module"]
pub type DHTMEM2 = crate::Reg<u32, _DHTMEM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM2;
#[doc = "`read()` method returns [dhtmem2::R](dhtmem2::R) reader structure"]
impl crate::Readable for DHTMEM2 {}
#[doc = "`write(|w| ..)` method takes [dhtmem2::W](dhtmem2::W) writer structure"]
impl crate::Writable for DHTMEM2 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem2;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem3](dhtmem3) module"]
pub type DHTMEM3 = crate::Reg<u32, _DHTMEM3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM3;
#[doc = "`read()` method returns [dhtmem3::R](dhtmem3::R) reader structure"]
impl crate::Readable for DHTMEM3 {}
#[doc = "`write(|w| ..)` method takes [dhtmem3::W](dhtmem3::W) writer structure"]
impl crate::Writable for DHTMEM3 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem3;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem4](dhtmem4) module"]
pub type DHTMEM4 = crate::Reg<u32, _DHTMEM4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM4;
#[doc = "`read()` method returns [dhtmem4::R](dhtmem4::R) reader structure"]
impl crate::Readable for DHTMEM4 {}
#[doc = "`write(|w| ..)` method takes [dhtmem4::W](dhtmem4::W) writer structure"]
impl crate::Writable for DHTMEM4 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem4;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem5](dhtmem5) module"]
pub type DHTMEM5 = crate::Reg<u32, _DHTMEM5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM5;
#[doc = "`read()` method returns [dhtmem5::R](dhtmem5::R) reader structure"]
impl crate::Readable for DHTMEM5 {}
#[doc = "`write(|w| ..)` method takes [dhtmem5::W](dhtmem5::W) writer structure"]
impl crate::Writable for DHTMEM5 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem5;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem6](dhtmem6) module"]
pub type DHTMEM6 = crate::Reg<u32, _DHTMEM6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM6;
#[doc = "`read()` method returns [dhtmem6::R](dhtmem6::R) reader structure"]
impl crate::Readable for DHTMEM6 {}
#[doc = "`write(|w| ..)` method takes [dhtmem6::W](dhtmem6::W) writer structure"]
impl crate::Writable for DHTMEM6 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem6;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem7](dhtmem7) module"]
pub type DHTMEM7 = crate::Reg<u32, _DHTMEM7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM7;
#[doc = "`read()` method returns [dhtmem7::R](dhtmem7::R) reader structure"]
impl crate::Readable for DHTMEM7 {}
#[doc = "`write(|w| ..)` method takes [dhtmem7::W](dhtmem7::W) writer structure"]
impl crate::Writable for DHTMEM7 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem7;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem8](dhtmem8) module"]
pub type DHTMEM8 = crate::Reg<u32, _DHTMEM8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM8;
#[doc = "`read()` method returns [dhtmem8::R](dhtmem8::R) reader structure"]
impl crate::Readable for DHTMEM8 {}
#[doc = "`write(|w| ..)` method takes [dhtmem8::W](dhtmem8::W) writer structure"]
impl crate::Writable for DHTMEM8 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem8;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem9](dhtmem9) module"]
pub type DHTMEM9 = crate::Reg<u32, _DHTMEM9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM9;
#[doc = "`read()` method returns [dhtmem9::R](dhtmem9::R) reader structure"]
impl crate::Readable for DHTMEM9 {}
#[doc = "`write(|w| ..)` method takes [dhtmem9::W](dhtmem9::W) writer structure"]
impl crate::Writable for DHTMEM9 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem9;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem10](dhtmem10) module"]
pub type DHTMEM10 = crate::Reg<u32, _DHTMEM10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM10;
#[doc = "`read()` method returns [dhtmem10::R](dhtmem10::R) reader structure"]
impl crate::Readable for DHTMEM10 {}
#[doc = "`write(|w| ..)` method takes [dhtmem10::W](dhtmem10::W) writer structure"]
impl crate::Writable for DHTMEM10 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem10;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem11](dhtmem11) module"]
pub type DHTMEM11 = crate::Reg<u32, _DHTMEM11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM11;
#[doc = "`read()` method returns [dhtmem11::R](dhtmem11::R) reader structure"]
impl crate::Readable for DHTMEM11 {}
#[doc = "`write(|w| ..)` method takes [dhtmem11::W](dhtmem11::W) writer structure"]
impl crate::Writable for DHTMEM11 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem11;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem12](dhtmem12) module"]
pub type DHTMEM12 = crate::Reg<u32, _DHTMEM12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM12;
#[doc = "`read()` method returns [dhtmem12::R](dhtmem12::R) reader structure"]
impl crate::Readable for DHTMEM12 {}
#[doc = "`write(|w| ..)` method takes [dhtmem12::W](dhtmem12::W) writer structure"]
impl crate::Writable for DHTMEM12 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem12;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem13](dhtmem13) module"]
pub type DHTMEM13 = crate::Reg<u32, _DHTMEM13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM13;
#[doc = "`read()` method returns [dhtmem13::R](dhtmem13::R) reader structure"]
impl crate::Readable for DHTMEM13 {}
#[doc = "`write(|w| ..)` method takes [dhtmem13::W](dhtmem13::W) writer structure"]
impl crate::Writable for DHTMEM13 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem13;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem14](dhtmem14) module"]
pub type DHTMEM14 = crate::Reg<u32, _DHTMEM14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM14;
#[doc = "`read()` method returns [dhtmem14::R](dhtmem14::R) reader structure"]
impl crate::Readable for DHTMEM14 {}
#[doc = "`write(|w| ..)` method takes [dhtmem14::W](dhtmem14::W) writer structure"]
impl crate::Writable for DHTMEM14 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem14;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem15](dhtmem15) module"]
pub type DHTMEM15 = crate::Reg<u32, _DHTMEM15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM15;
#[doc = "`read()` method returns [dhtmem15::R](dhtmem15::R) reader structure"]
impl crate::Readable for DHTMEM15 {}
#[doc = "`write(|w| ..)` method takes [dhtmem15::W](dhtmem15::W) writer structure"]
impl crate::Writable for DHTMEM15 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem15;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem16](dhtmem16) module"]
pub type DHTMEM16 = crate::Reg<u32, _DHTMEM16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM16;
#[doc = "`read()` method returns [dhtmem16::R](dhtmem16::R) reader structure"]
impl crate::Readable for DHTMEM16 {}
#[doc = "`write(|w| ..)` method takes [dhtmem16::W](dhtmem16::W) writer structure"]
impl crate::Writable for DHTMEM16 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem16;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem17](dhtmem17) module"]
pub type DHTMEM17 = crate::Reg<u32, _DHTMEM17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM17;
#[doc = "`read()` method returns [dhtmem17::R](dhtmem17::R) reader structure"]
impl crate::Readable for DHTMEM17 {}
#[doc = "`write(|w| ..)` method takes [dhtmem17::W](dhtmem17::W) writer structure"]
impl crate::Writable for DHTMEM17 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem17;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem18](dhtmem18) module"]
pub type DHTMEM18 = crate::Reg<u32, _DHTMEM18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM18;
#[doc = "`read()` method returns [dhtmem18::R](dhtmem18::R) reader structure"]
impl crate::Readable for DHTMEM18 {}
#[doc = "`write(|w| ..)` method takes [dhtmem18::W](dhtmem18::W) writer structure"]
impl crate::Writable for DHTMEM18 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem18;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem19](dhtmem19) module"]
pub type DHTMEM19 = crate::Reg<u32, _DHTMEM19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM19;
#[doc = "`read()` method returns [dhtmem19::R](dhtmem19::R) reader structure"]
impl crate::Readable for DHTMEM19 {}
#[doc = "`write(|w| ..)` method takes [dhtmem19::W](dhtmem19::W) writer structure"]
impl crate::Writable for DHTMEM19 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem19;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem20](dhtmem20) module"]
pub type DHTMEM20 = crate::Reg<u32, _DHTMEM20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM20;
#[doc = "`read()` method returns [dhtmem20::R](dhtmem20::R) reader structure"]
impl crate::Readable for DHTMEM20 {}
#[doc = "`write(|w| ..)` method takes [dhtmem20::W](dhtmem20::W) writer structure"]
impl crate::Writable for DHTMEM20 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem20;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem21](dhtmem21) module"]
pub type DHTMEM21 = crate::Reg<u32, _DHTMEM21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM21;
#[doc = "`read()` method returns [dhtmem21::R](dhtmem21::R) reader structure"]
impl crate::Readable for DHTMEM21 {}
#[doc = "`write(|w| ..)` method takes [dhtmem21::W](dhtmem21::W) writer structure"]
impl crate::Writable for DHTMEM21 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem21;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem22](dhtmem22) module"]
pub type DHTMEM22 = crate::Reg<u32, _DHTMEM22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM22;
#[doc = "`read()` method returns [dhtmem22::R](dhtmem22::R) reader structure"]
impl crate::Readable for DHTMEM22 {}
#[doc = "`write(|w| ..)` method takes [dhtmem22::W](dhtmem22::W) writer structure"]
impl crate::Writable for DHTMEM22 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem22;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem23](dhtmem23) module"]
pub type DHTMEM23 = crate::Reg<u32, _DHTMEM23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM23;
#[doc = "`read()` method returns [dhtmem23::R](dhtmem23::R) reader structure"]
impl crate::Readable for DHTMEM23 {}
#[doc = "`write(|w| ..)` method takes [dhtmem23::W](dhtmem23::W) writer structure"]
impl crate::Writable for DHTMEM23 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem23;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem24](dhtmem24) module"]
pub type DHTMEM24 = crate::Reg<u32, _DHTMEM24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM24;
#[doc = "`read()` method returns [dhtmem24::R](dhtmem24::R) reader structure"]
impl crate::Readable for DHTMEM24 {}
#[doc = "`write(|w| ..)` method takes [dhtmem24::W](dhtmem24::W) writer structure"]
impl crate::Writable for DHTMEM24 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem24;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem25](dhtmem25) module"]
pub type DHTMEM25 = crate::Reg<u32, _DHTMEM25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM25;
#[doc = "`read()` method returns [dhtmem25::R](dhtmem25::R) reader structure"]
impl crate::Readable for DHTMEM25 {}
#[doc = "`write(|w| ..)` method takes [dhtmem25::W](dhtmem25::W) writer structure"]
impl crate::Writable for DHTMEM25 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem25;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem26](dhtmem26) module"]
pub type DHTMEM26 = crate::Reg<u32, _DHTMEM26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM26;
#[doc = "`read()` method returns [dhtmem26::R](dhtmem26::R) reader structure"]
impl crate::Readable for DHTMEM26 {}
#[doc = "`write(|w| ..)` method takes [dhtmem26::W](dhtmem26::W) writer structure"]
impl crate::Writable for DHTMEM26 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem26;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem27](dhtmem27) module"]
pub type DHTMEM27 = crate::Reg<u32, _DHTMEM27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM27;
#[doc = "`read()` method returns [dhtmem27::R](dhtmem27::R) reader structure"]
impl crate::Readable for DHTMEM27 {}
#[doc = "`write(|w| ..)` method takes [dhtmem27::W](dhtmem27::W) writer structure"]
impl crate::Writable for DHTMEM27 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem27;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem28](dhtmem28) module"]
pub type DHTMEM28 = crate::Reg<u32, _DHTMEM28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM28;
#[doc = "`read()` method returns [dhtmem28::R](dhtmem28::R) reader structure"]
impl crate::Readable for DHTMEM28 {}
#[doc = "`write(|w| ..)` method takes [dhtmem28::W](dhtmem28::W) writer structure"]
impl crate::Writable for DHTMEM28 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem28;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem29](dhtmem29) module"]
pub type DHTMEM29 = crate::Reg<u32, _DHTMEM29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM29;
#[doc = "`read()` method returns [dhtmem29::R](dhtmem29::R) reader structure"]
impl crate::Readable for DHTMEM29 {}
#[doc = "`write(|w| ..)` method takes [dhtmem29::W](dhtmem29::W) writer structure"]
impl crate::Writable for DHTMEM29 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem29;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem30](dhtmem30) module"]
pub type DHTMEM30 = crate::Reg<u32, _DHTMEM30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM30;
#[doc = "`read()` method returns [dhtmem30::R](dhtmem30::R) reader structure"]
impl crate::Readable for DHTMEM30 {}
#[doc = "`write(|w| ..)` method takes [dhtmem30::W](dhtmem30::W) writer structure"]
impl crate::Writable for DHTMEM30 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem30;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem31](dhtmem31) module"]
pub type DHTMEM31 = crate::Reg<u32, _DHTMEM31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM31;
#[doc = "`read()` method returns [dhtmem31::R](dhtmem31::R) reader structure"]
impl crate::Readable for DHTMEM31 {}
#[doc = "`write(|w| ..)` method takes [dhtmem31::W](dhtmem31::W) writer structure"]
impl crate::Writable for DHTMEM31 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem31;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem32](dhtmem32) module"]
pub type DHTMEM32 = crate::Reg<u32, _DHTMEM32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM32;
#[doc = "`read()` method returns [dhtmem32::R](dhtmem32::R) reader structure"]
impl crate::Readable for DHTMEM32 {}
#[doc = "`write(|w| ..)` method takes [dhtmem32::W](dhtmem32::W) writer structure"]
impl crate::Writable for DHTMEM32 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem32;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem33](dhtmem33) module"]
pub type DHTMEM33 = crate::Reg<u32, _DHTMEM33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM33;
#[doc = "`read()` method returns [dhtmem33::R](dhtmem33::R) reader structure"]
impl crate::Readable for DHTMEM33 {}
#[doc = "`write(|w| ..)` method takes [dhtmem33::W](dhtmem33::W) writer structure"]
impl crate::Writable for DHTMEM33 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem33;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem34](dhtmem34) module"]
pub type DHTMEM34 = crate::Reg<u32, _DHTMEM34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM34;
#[doc = "`read()` method returns [dhtmem34::R](dhtmem34::R) reader structure"]
impl crate::Readable for DHTMEM34 {}
#[doc = "`write(|w| ..)` method takes [dhtmem34::W](dhtmem34::W) writer structure"]
impl crate::Writable for DHTMEM34 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem34;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem35](dhtmem35) module"]
pub type DHTMEM35 = crate::Reg<u32, _DHTMEM35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM35;
#[doc = "`read()` method returns [dhtmem35::R](dhtmem35::R) reader structure"]
impl crate::Readable for DHTMEM35 {}
#[doc = "`write(|w| ..)` method takes [dhtmem35::W](dhtmem35::W) writer structure"]
impl crate::Writable for DHTMEM35 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem35;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem36](dhtmem36) module"]
pub type DHTMEM36 = crate::Reg<u32, _DHTMEM36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM36;
#[doc = "`read()` method returns [dhtmem36::R](dhtmem36::R) reader structure"]
impl crate::Readable for DHTMEM36 {}
#[doc = "`write(|w| ..)` method takes [dhtmem36::W](dhtmem36::W) writer structure"]
impl crate::Writable for DHTMEM36 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem36;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem37](dhtmem37) module"]
pub type DHTMEM37 = crate::Reg<u32, _DHTMEM37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM37;
#[doc = "`read()` method returns [dhtmem37::R](dhtmem37::R) reader structure"]
impl crate::Readable for DHTMEM37 {}
#[doc = "`write(|w| ..)` method takes [dhtmem37::W](dhtmem37::W) writer structure"]
impl crate::Writable for DHTMEM37 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem37;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem38](dhtmem38) module"]
pub type DHTMEM38 = crate::Reg<u32, _DHTMEM38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM38;
#[doc = "`read()` method returns [dhtmem38::R](dhtmem38::R) reader structure"]
impl crate::Readable for DHTMEM38 {}
#[doc = "`write(|w| ..)` method takes [dhtmem38::W](dhtmem38::W) writer structure"]
impl crate::Writable for DHTMEM38 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem38;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem39](dhtmem39) module"]
pub type DHTMEM39 = crate::Reg<u32, _DHTMEM39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM39;
#[doc = "`read()` method returns [dhtmem39::R](dhtmem39::R) reader structure"]
impl crate::Readable for DHTMEM39 {}
#[doc = "`write(|w| ..)` method takes [dhtmem39::W](dhtmem39::W) writer structure"]
impl crate::Writable for DHTMEM39 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem39;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem40](dhtmem40) module"]
pub type DHTMEM40 = crate::Reg<u32, _DHTMEM40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM40;
#[doc = "`read()` method returns [dhtmem40::R](dhtmem40::R) reader structure"]
impl crate::Readable for DHTMEM40 {}
#[doc = "`write(|w| ..)` method takes [dhtmem40::W](dhtmem40::W) writer structure"]
impl crate::Writable for DHTMEM40 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem40;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem41](dhtmem41) module"]
pub type DHTMEM41 = crate::Reg<u32, _DHTMEM41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM41;
#[doc = "`read()` method returns [dhtmem41::R](dhtmem41::R) reader structure"]
impl crate::Readable for DHTMEM41 {}
#[doc = "`write(|w| ..)` method takes [dhtmem41::W](dhtmem41::W) writer structure"]
impl crate::Writable for DHTMEM41 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem41;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem42](dhtmem42) module"]
pub type DHTMEM42 = crate::Reg<u32, _DHTMEM42>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM42;
#[doc = "`read()` method returns [dhtmem42::R](dhtmem42::R) reader structure"]
impl crate::Readable for DHTMEM42 {}
#[doc = "`write(|w| ..)` method takes [dhtmem42::W](dhtmem42::W) writer structure"]
impl crate::Writable for DHTMEM42 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem42;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem43](dhtmem43) module"]
pub type DHTMEM43 = crate::Reg<u32, _DHTMEM43>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM43;
#[doc = "`read()` method returns [dhtmem43::R](dhtmem43::R) reader structure"]
impl crate::Readable for DHTMEM43 {}
#[doc = "`write(|w| ..)` method takes [dhtmem43::W](dhtmem43::W) writer structure"]
impl crate::Writable for DHTMEM43 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem43;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem44](dhtmem44) module"]
pub type DHTMEM44 = crate::Reg<u32, _DHTMEM44>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM44;
#[doc = "`read()` method returns [dhtmem44::R](dhtmem44::R) reader structure"]
impl crate::Readable for DHTMEM44 {}
#[doc = "`write(|w| ..)` method takes [dhtmem44::W](dhtmem44::W) writer structure"]
impl crate::Writable for DHTMEM44 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem44;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem45](dhtmem45) module"]
pub type DHTMEM45 = crate::Reg<u32, _DHTMEM45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM45;
#[doc = "`read()` method returns [dhtmem45::R](dhtmem45::R) reader structure"]
impl crate::Readable for DHTMEM45 {}
#[doc = "`write(|w| ..)` method takes [dhtmem45::W](dhtmem45::W) writer structure"]
impl crate::Writable for DHTMEM45 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem45;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem46](dhtmem46) module"]
pub type DHTMEM46 = crate::Reg<u32, _DHTMEM46>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM46;
#[doc = "`read()` method returns [dhtmem46::R](dhtmem46::R) reader structure"]
impl crate::Readable for DHTMEM46 {}
#[doc = "`write(|w| ..)` method takes [dhtmem46::W](dhtmem46::W) writer structure"]
impl crate::Writable for DHTMEM46 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem46;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem47](dhtmem47) module"]
pub type DHTMEM47 = crate::Reg<u32, _DHTMEM47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM47;
#[doc = "`read()` method returns [dhtmem47::R](dhtmem47::R) reader structure"]
impl crate::Readable for DHTMEM47 {}
#[doc = "`write(|w| ..)` method takes [dhtmem47::W](dhtmem47::W) writer structure"]
impl crate::Writable for DHTMEM47 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem47;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem48](dhtmem48) module"]
pub type DHTMEM48 = crate::Reg<u32, _DHTMEM48>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM48;
#[doc = "`read()` method returns [dhtmem48::R](dhtmem48::R) reader structure"]
impl crate::Readable for DHTMEM48 {}
#[doc = "`write(|w| ..)` method takes [dhtmem48::W](dhtmem48::W) writer structure"]
impl crate::Writable for DHTMEM48 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem48;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem49](dhtmem49) module"]
pub type DHTMEM49 = crate::Reg<u32, _DHTMEM49>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM49;
#[doc = "`read()` method returns [dhtmem49::R](dhtmem49::R) reader structure"]
impl crate::Readable for DHTMEM49 {}
#[doc = "`write(|w| ..)` method takes [dhtmem49::W](dhtmem49::W) writer structure"]
impl crate::Writable for DHTMEM49 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem49;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem50](dhtmem50) module"]
pub type DHTMEM50 = crate::Reg<u32, _DHTMEM50>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM50;
#[doc = "`read()` method returns [dhtmem50::R](dhtmem50::R) reader structure"]
impl crate::Readable for DHTMEM50 {}
#[doc = "`write(|w| ..)` method takes [dhtmem50::W](dhtmem50::W) writer structure"]
impl crate::Writable for DHTMEM50 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem50;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem51](dhtmem51) module"]
pub type DHTMEM51 = crate::Reg<u32, _DHTMEM51>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM51;
#[doc = "`read()` method returns [dhtmem51::R](dhtmem51::R) reader structure"]
impl crate::Readable for DHTMEM51 {}
#[doc = "`write(|w| ..)` method takes [dhtmem51::W](dhtmem51::W) writer structure"]
impl crate::Writable for DHTMEM51 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem51;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem52](dhtmem52) module"]
pub type DHTMEM52 = crate::Reg<u32, _DHTMEM52>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM52;
#[doc = "`read()` method returns [dhtmem52::R](dhtmem52::R) reader structure"]
impl crate::Readable for DHTMEM52 {}
#[doc = "`write(|w| ..)` method takes [dhtmem52::W](dhtmem52::W) writer structure"]
impl crate::Writable for DHTMEM52 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem52;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem53](dhtmem53) module"]
pub type DHTMEM53 = crate::Reg<u32, _DHTMEM53>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM53;
#[doc = "`read()` method returns [dhtmem53::R](dhtmem53::R) reader structure"]
impl crate::Readable for DHTMEM53 {}
#[doc = "`write(|w| ..)` method takes [dhtmem53::W](dhtmem53::W) writer structure"]
impl crate::Writable for DHTMEM53 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem53;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem54](dhtmem54) module"]
pub type DHTMEM54 = crate::Reg<u32, _DHTMEM54>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM54;
#[doc = "`read()` method returns [dhtmem54::R](dhtmem54::R) reader structure"]
impl crate::Readable for DHTMEM54 {}
#[doc = "`write(|w| ..)` method takes [dhtmem54::W](dhtmem54::W) writer structure"]
impl crate::Writable for DHTMEM54 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem54;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem55](dhtmem55) module"]
pub type DHTMEM55 = crate::Reg<u32, _DHTMEM55>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM55;
#[doc = "`read()` method returns [dhtmem55::R](dhtmem55::R) reader structure"]
impl crate::Readable for DHTMEM55 {}
#[doc = "`write(|w| ..)` method takes [dhtmem55::W](dhtmem55::W) writer structure"]
impl crate::Writable for DHTMEM55 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem55;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem56](dhtmem56) module"]
pub type DHTMEM56 = crate::Reg<u32, _DHTMEM56>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM56;
#[doc = "`read()` method returns [dhtmem56::R](dhtmem56::R) reader structure"]
impl crate::Readable for DHTMEM56 {}
#[doc = "`write(|w| ..)` method takes [dhtmem56::W](dhtmem56::W) writer structure"]
impl crate::Writable for DHTMEM56 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem56;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem57](dhtmem57) module"]
pub type DHTMEM57 = crate::Reg<u32, _DHTMEM57>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM57;
#[doc = "`read()` method returns [dhtmem57::R](dhtmem57::R) reader structure"]
impl crate::Readable for DHTMEM57 {}
#[doc = "`write(|w| ..)` method takes [dhtmem57::W](dhtmem57::W) writer structure"]
impl crate::Writable for DHTMEM57 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem57;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem58](dhtmem58) module"]
pub type DHTMEM58 = crate::Reg<u32, _DHTMEM58>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM58;
#[doc = "`read()` method returns [dhtmem58::R](dhtmem58::R) reader structure"]
impl crate::Readable for DHTMEM58 {}
#[doc = "`write(|w| ..)` method takes [dhtmem58::W](dhtmem58::W) writer structure"]
impl crate::Writable for DHTMEM58 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem58;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem59](dhtmem59) module"]
pub type DHTMEM59 = crate::Reg<u32, _DHTMEM59>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM59;
#[doc = "`read()` method returns [dhtmem59::R](dhtmem59::R) reader structure"]
impl crate::Readable for DHTMEM59 {}
#[doc = "`write(|w| ..)` method takes [dhtmem59::W](dhtmem59::W) writer structure"]
impl crate::Writable for DHTMEM59 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem59;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem60](dhtmem60) module"]
pub type DHTMEM60 = crate::Reg<u32, _DHTMEM60>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM60;
#[doc = "`read()` method returns [dhtmem60::R](dhtmem60::R) reader structure"]
impl crate::Readable for DHTMEM60 {}
#[doc = "`write(|w| ..)` method takes [dhtmem60::W](dhtmem60::W) writer structure"]
impl crate::Writable for DHTMEM60 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem60;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem61](dhtmem61) module"]
pub type DHTMEM61 = crate::Reg<u32, _DHTMEM61>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM61;
#[doc = "`read()` method returns [dhtmem61::R](dhtmem61::R) reader structure"]
impl crate::Readable for DHTMEM61 {}
#[doc = "`write(|w| ..)` method takes [dhtmem61::W](dhtmem61::W) writer structure"]
impl crate::Writable for DHTMEM61 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem61;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem62](dhtmem62) module"]
pub type DHTMEM62 = crate::Reg<u32, _DHTMEM62>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM62;
#[doc = "`read()` method returns [dhtmem62::R](dhtmem62::R) reader structure"]
impl crate::Readable for DHTMEM62 {}
#[doc = "`write(|w| ..)` method takes [dhtmem62::W](dhtmem62::W) writer structure"]
impl crate::Writable for DHTMEM62 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem62;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem63](dhtmem63) module"]
pub type DHTMEM63 = crate::Reg<u32, _DHTMEM63>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM63;
#[doc = "`read()` method returns [dhtmem63::R](dhtmem63::R) reader structure"]
impl crate::Readable for DHTMEM63 {}
#[doc = "`write(|w| ..)` method takes [dhtmem63::W](dhtmem63::W) writer structure"]
impl crate::Writable for DHTMEM63 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem63;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem64](dhtmem64) module"]
pub type DHTMEM64 = crate::Reg<u32, _DHTMEM64>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM64;
#[doc = "`read()` method returns [dhtmem64::R](dhtmem64::R) reader structure"]
impl crate::Readable for DHTMEM64 {}
#[doc = "`write(|w| ..)` method takes [dhtmem64::W](dhtmem64::W) writer structure"]
impl crate::Writable for DHTMEM64 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem64;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem65](dhtmem65) module"]
pub type DHTMEM65 = crate::Reg<u32, _DHTMEM65>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM65;
#[doc = "`read()` method returns [dhtmem65::R](dhtmem65::R) reader structure"]
impl crate::Readable for DHTMEM65 {}
#[doc = "`write(|w| ..)` method takes [dhtmem65::W](dhtmem65::W) writer structure"]
impl crate::Writable for DHTMEM65 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem65;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem66](dhtmem66) module"]
pub type DHTMEM66 = crate::Reg<u32, _DHTMEM66>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM66;
#[doc = "`read()` method returns [dhtmem66::R](dhtmem66::R) reader structure"]
impl crate::Readable for DHTMEM66 {}
#[doc = "`write(|w| ..)` method takes [dhtmem66::W](dhtmem66::W) writer structure"]
impl crate::Writable for DHTMEM66 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem66;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem67](dhtmem67) module"]
pub type DHTMEM67 = crate::Reg<u32, _DHTMEM67>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM67;
#[doc = "`read()` method returns [dhtmem67::R](dhtmem67::R) reader structure"]
impl crate::Readable for DHTMEM67 {}
#[doc = "`write(|w| ..)` method takes [dhtmem67::W](dhtmem67::W) writer structure"]
impl crate::Writable for DHTMEM67 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem67;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem68](dhtmem68) module"]
pub type DHTMEM68 = crate::Reg<u32, _DHTMEM68>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM68;
#[doc = "`read()` method returns [dhtmem68::R](dhtmem68::R) reader structure"]
impl crate::Readable for DHTMEM68 {}
#[doc = "`write(|w| ..)` method takes [dhtmem68::W](dhtmem68::W) writer structure"]
impl crate::Writable for DHTMEM68 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem68;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem69](dhtmem69) module"]
pub type DHTMEM69 = crate::Reg<u32, _DHTMEM69>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM69;
#[doc = "`read()` method returns [dhtmem69::R](dhtmem69::R) reader structure"]
impl crate::Readable for DHTMEM69 {}
#[doc = "`write(|w| ..)` method takes [dhtmem69::W](dhtmem69::W) writer structure"]
impl crate::Writable for DHTMEM69 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem69;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem70](dhtmem70) module"]
pub type DHTMEM70 = crate::Reg<u32, _DHTMEM70>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM70;
#[doc = "`read()` method returns [dhtmem70::R](dhtmem70::R) reader structure"]
impl crate::Readable for DHTMEM70 {}
#[doc = "`write(|w| ..)` method takes [dhtmem70::W](dhtmem70::W) writer structure"]
impl crate::Writable for DHTMEM70 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem70;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem71](dhtmem71) module"]
pub type DHTMEM71 = crate::Reg<u32, _DHTMEM71>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM71;
#[doc = "`read()` method returns [dhtmem71::R](dhtmem71::R) reader structure"]
impl crate::Readable for DHTMEM71 {}
#[doc = "`write(|w| ..)` method takes [dhtmem71::W](dhtmem71::W) writer structure"]
impl crate::Writable for DHTMEM71 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem71;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem72](dhtmem72) module"]
pub type DHTMEM72 = crate::Reg<u32, _DHTMEM72>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM72;
#[doc = "`read()` method returns [dhtmem72::R](dhtmem72::R) reader structure"]
impl crate::Readable for DHTMEM72 {}
#[doc = "`write(|w| ..)` method takes [dhtmem72::W](dhtmem72::W) writer structure"]
impl crate::Writable for DHTMEM72 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem72;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem73](dhtmem73) module"]
pub type DHTMEM73 = crate::Reg<u32, _DHTMEM73>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM73;
#[doc = "`read()` method returns [dhtmem73::R](dhtmem73::R) reader structure"]
impl crate::Readable for DHTMEM73 {}
#[doc = "`write(|w| ..)` method takes [dhtmem73::W](dhtmem73::W) writer structure"]
impl crate::Writable for DHTMEM73 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem73;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem74](dhtmem74) module"]
pub type DHTMEM74 = crate::Reg<u32, _DHTMEM74>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM74;
#[doc = "`read()` method returns [dhtmem74::R](dhtmem74::R) reader structure"]
impl crate::Readable for DHTMEM74 {}
#[doc = "`write(|w| ..)` method takes [dhtmem74::W](dhtmem74::W) writer structure"]
impl crate::Writable for DHTMEM74 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem74;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem75](dhtmem75) module"]
pub type DHTMEM75 = crate::Reg<u32, _DHTMEM75>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM75;
#[doc = "`read()` method returns [dhtmem75::R](dhtmem75::R) reader structure"]
impl crate::Readable for DHTMEM75 {}
#[doc = "`write(|w| ..)` method takes [dhtmem75::W](dhtmem75::W) writer structure"]
impl crate::Writable for DHTMEM75 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem75;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem76](dhtmem76) module"]
pub type DHTMEM76 = crate::Reg<u32, _DHTMEM76>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM76;
#[doc = "`read()` method returns [dhtmem76::R](dhtmem76::R) reader structure"]
impl crate::Readable for DHTMEM76 {}
#[doc = "`write(|w| ..)` method takes [dhtmem76::W](dhtmem76::W) writer structure"]
impl crate::Writable for DHTMEM76 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem76;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem77](dhtmem77) module"]
pub type DHTMEM77 = crate::Reg<u32, _DHTMEM77>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM77;
#[doc = "`read()` method returns [dhtmem77::R](dhtmem77::R) reader structure"]
impl crate::Readable for DHTMEM77 {}
#[doc = "`write(|w| ..)` method takes [dhtmem77::W](dhtmem77::W) writer structure"]
impl crate::Writable for DHTMEM77 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem77;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem78](dhtmem78) module"]
pub type DHTMEM78 = crate::Reg<u32, _DHTMEM78>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM78;
#[doc = "`read()` method returns [dhtmem78::R](dhtmem78::R) reader structure"]
impl crate::Readable for DHTMEM78 {}
#[doc = "`write(|w| ..)` method takes [dhtmem78::W](dhtmem78::W) writer structure"]
impl crate::Writable for DHTMEM78 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem78;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem79](dhtmem79) module"]
pub type DHTMEM79 = crate::Reg<u32, _DHTMEM79>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM79;
#[doc = "`read()` method returns [dhtmem79::R](dhtmem79::R) reader structure"]
impl crate::Readable for DHTMEM79 {}
#[doc = "`write(|w| ..)` method takes [dhtmem79::W](dhtmem79::W) writer structure"]
impl crate::Writable for DHTMEM79 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem79;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem80](dhtmem80) module"]
pub type DHTMEM80 = crate::Reg<u32, _DHTMEM80>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM80;
#[doc = "`read()` method returns [dhtmem80::R](dhtmem80::R) reader structure"]
impl crate::Readable for DHTMEM80 {}
#[doc = "`write(|w| ..)` method takes [dhtmem80::W](dhtmem80::W) writer structure"]
impl crate::Writable for DHTMEM80 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem80;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem81](dhtmem81) module"]
pub type DHTMEM81 = crate::Reg<u32, _DHTMEM81>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM81;
#[doc = "`read()` method returns [dhtmem81::R](dhtmem81::R) reader structure"]
impl crate::Readable for DHTMEM81 {}
#[doc = "`write(|w| ..)` method takes [dhtmem81::W](dhtmem81::W) writer structure"]
impl crate::Writable for DHTMEM81 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem81;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem82](dhtmem82) module"]
pub type DHTMEM82 = crate::Reg<u32, _DHTMEM82>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM82;
#[doc = "`read()` method returns [dhtmem82::R](dhtmem82::R) reader structure"]
impl crate::Readable for DHTMEM82 {}
#[doc = "`write(|w| ..)` method takes [dhtmem82::W](dhtmem82::W) writer structure"]
impl crate::Writable for DHTMEM82 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem82;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem83](dhtmem83) module"]
pub type DHTMEM83 = crate::Reg<u32, _DHTMEM83>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM83;
#[doc = "`read()` method returns [dhtmem83::R](dhtmem83::R) reader structure"]
impl crate::Readable for DHTMEM83 {}
#[doc = "`write(|w| ..)` method takes [dhtmem83::W](dhtmem83::W) writer structure"]
impl crate::Writable for DHTMEM83 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem83;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem84](dhtmem84) module"]
pub type DHTMEM84 = crate::Reg<u32, _DHTMEM84>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM84;
#[doc = "`read()` method returns [dhtmem84::R](dhtmem84::R) reader structure"]
impl crate::Readable for DHTMEM84 {}
#[doc = "`write(|w| ..)` method takes [dhtmem84::W](dhtmem84::W) writer structure"]
impl crate::Writable for DHTMEM84 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem84;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem85](dhtmem85) module"]
pub type DHTMEM85 = crate::Reg<u32, _DHTMEM85>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM85;
#[doc = "`read()` method returns [dhtmem85::R](dhtmem85::R) reader structure"]
impl crate::Readable for DHTMEM85 {}
#[doc = "`write(|w| ..)` method takes [dhtmem85::W](dhtmem85::W) writer structure"]
impl crate::Writable for DHTMEM85 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem85;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem86](dhtmem86) module"]
pub type DHTMEM86 = crate::Reg<u32, _DHTMEM86>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM86;
#[doc = "`read()` method returns [dhtmem86::R](dhtmem86::R) reader structure"]
impl crate::Readable for DHTMEM86 {}
#[doc = "`write(|w| ..)` method takes [dhtmem86::W](dhtmem86::W) writer structure"]
impl crate::Writable for DHTMEM86 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem86;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem87](dhtmem87) module"]
pub type DHTMEM87 = crate::Reg<u32, _DHTMEM87>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM87;
#[doc = "`read()` method returns [dhtmem87::R](dhtmem87::R) reader structure"]
impl crate::Readable for DHTMEM87 {}
#[doc = "`write(|w| ..)` method takes [dhtmem87::W](dhtmem87::W) writer structure"]
impl crate::Writable for DHTMEM87 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem87;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem88](dhtmem88) module"]
pub type DHTMEM88 = crate::Reg<u32, _DHTMEM88>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM88;
#[doc = "`read()` method returns [dhtmem88::R](dhtmem88::R) reader structure"]
impl crate::Readable for DHTMEM88 {}
#[doc = "`write(|w| ..)` method takes [dhtmem88::W](dhtmem88::W) writer structure"]
impl crate::Writable for DHTMEM88 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem88;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem89](dhtmem89) module"]
pub type DHTMEM89 = crate::Reg<u32, _DHTMEM89>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM89;
#[doc = "`read()` method returns [dhtmem89::R](dhtmem89::R) reader structure"]
impl crate::Readable for DHTMEM89 {}
#[doc = "`write(|w| ..)` method takes [dhtmem89::W](dhtmem89::W) writer structure"]
impl crate::Writable for DHTMEM89 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem89;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem90](dhtmem90) module"]
pub type DHTMEM90 = crate::Reg<u32, _DHTMEM90>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM90;
#[doc = "`read()` method returns [dhtmem90::R](dhtmem90::R) reader structure"]
impl crate::Readable for DHTMEM90 {}
#[doc = "`write(|w| ..)` method takes [dhtmem90::W](dhtmem90::W) writer structure"]
impl crate::Writable for DHTMEM90 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem90;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem91](dhtmem91) module"]
pub type DHTMEM91 = crate::Reg<u32, _DHTMEM91>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM91;
#[doc = "`read()` method returns [dhtmem91::R](dhtmem91::R) reader structure"]
impl crate::Readable for DHTMEM91 {}
#[doc = "`write(|w| ..)` method takes [dhtmem91::W](dhtmem91::W) writer structure"]
impl crate::Writable for DHTMEM91 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem91;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem92](dhtmem92) module"]
pub type DHTMEM92 = crate::Reg<u32, _DHTMEM92>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM92;
#[doc = "`read()` method returns [dhtmem92::R](dhtmem92::R) reader structure"]
impl crate::Readable for DHTMEM92 {}
#[doc = "`write(|w| ..)` method takes [dhtmem92::W](dhtmem92::W) writer structure"]
impl crate::Writable for DHTMEM92 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem92;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem93](dhtmem93) module"]
pub type DHTMEM93 = crate::Reg<u32, _DHTMEM93>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM93;
#[doc = "`read()` method returns [dhtmem93::R](dhtmem93::R) reader structure"]
impl crate::Readable for DHTMEM93 {}
#[doc = "`write(|w| ..)` method takes [dhtmem93::W](dhtmem93::W) writer structure"]
impl crate::Writable for DHTMEM93 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem93;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem94](dhtmem94) module"]
pub type DHTMEM94 = crate::Reg<u32, _DHTMEM94>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM94;
#[doc = "`read()` method returns [dhtmem94::R](dhtmem94::R) reader structure"]
impl crate::Readable for DHTMEM94 {}
#[doc = "`write(|w| ..)` method takes [dhtmem94::W](dhtmem94::W) writer structure"]
impl crate::Writable for DHTMEM94 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem94;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem95](dhtmem95) module"]
pub type DHTMEM95 = crate::Reg<u32, _DHTMEM95>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM95;
#[doc = "`read()` method returns [dhtmem95::R](dhtmem95::R) reader structure"]
impl crate::Readable for DHTMEM95 {}
#[doc = "`write(|w| ..)` method takes [dhtmem95::W](dhtmem95::W) writer structure"]
impl crate::Writable for DHTMEM95 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem95;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem96](dhtmem96) module"]
pub type DHTMEM96 = crate::Reg<u32, _DHTMEM96>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM96;
#[doc = "`read()` method returns [dhtmem96::R](dhtmem96::R) reader structure"]
impl crate::Readable for DHTMEM96 {}
#[doc = "`write(|w| ..)` method takes [dhtmem96::W](dhtmem96::W) writer structure"]
impl crate::Writable for DHTMEM96 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem96;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem97](dhtmem97) module"]
pub type DHTMEM97 = crate::Reg<u32, _DHTMEM97>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM97;
#[doc = "`read()` method returns [dhtmem97::R](dhtmem97::R) reader structure"]
impl crate::Readable for DHTMEM97 {}
#[doc = "`write(|w| ..)` method takes [dhtmem97::W](dhtmem97::W) writer structure"]
impl crate::Writable for DHTMEM97 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem97;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem98](dhtmem98) module"]
pub type DHTMEM98 = crate::Reg<u32, _DHTMEM98>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM98;
#[doc = "`read()` method returns [dhtmem98::R](dhtmem98::R) reader structure"]
impl crate::Readable for DHTMEM98 {}
#[doc = "`write(|w| ..)` method takes [dhtmem98::W](dhtmem98::W) writer structure"]
impl crate::Writable for DHTMEM98 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem98;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem99](dhtmem99) module"]
pub type DHTMEM99 = crate::Reg<u32, _DHTMEM99>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM99;
#[doc = "`read()` method returns [dhtmem99::R](dhtmem99::R) reader structure"]
impl crate::Readable for DHTMEM99 {}
#[doc = "`write(|w| ..)` method takes [dhtmem99::W](dhtmem99::W) writer structure"]
impl crate::Writable for DHTMEM99 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem99;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem100](dhtmem100) module"]
pub type DHTMEM100 = crate::Reg<u32, _DHTMEM100>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM100;
#[doc = "`read()` method returns [dhtmem100::R](dhtmem100::R) reader structure"]
impl crate::Readable for DHTMEM100 {}
#[doc = "`write(|w| ..)` method takes [dhtmem100::W](dhtmem100::W) writer structure"]
impl crate::Writable for DHTMEM100 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem100;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem101](dhtmem101) module"]
pub type DHTMEM101 = crate::Reg<u32, _DHTMEM101>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM101;
#[doc = "`read()` method returns [dhtmem101::R](dhtmem101::R) reader structure"]
impl crate::Readable for DHTMEM101 {}
#[doc = "`write(|w| ..)` method takes [dhtmem101::W](dhtmem101::W) writer structure"]
impl crate::Writable for DHTMEM101 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem101;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem102](dhtmem102) module"]
pub type DHTMEM102 = crate::Reg<u32, _DHTMEM102>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM102;
#[doc = "`read()` method returns [dhtmem102::R](dhtmem102::R) reader structure"]
impl crate::Readable for DHTMEM102 {}
#[doc = "`write(|w| ..)` method takes [dhtmem102::W](dhtmem102::W) writer structure"]
impl crate::Writable for DHTMEM102 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem102;
#[doc = "JPEG DHTMem tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhtmem103](dhtmem103) module"]
pub type DHTMEM103 = crate::Reg<u32, _DHTMEM103>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHTMEM103;
#[doc = "`read()` method returns [dhtmem103::R](dhtmem103::R) reader structure"]
impl crate::Readable for DHTMEM103 {}
#[doc = "`write(|w| ..)` method takes [dhtmem103::W](dhtmem103::W) writer structure"]
impl crate::Writable for DHTMEM103 {}
#[doc = "JPEG DHTMem tables"]
pub mod dhtmem103;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_0](huffenc_ac0_0) module"]
pub type HUFFENC_AC0_0 = crate::Reg<u32, _HUFFENC_AC0_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_0;
#[doc = "`read()` method returns [huffenc_ac0_0::R](huffenc_ac0_0::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_0 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_0::W](huffenc_ac0_0::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_0 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_0;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_1](huffenc_ac0_1) module"]
pub type HUFFENC_AC0_1 = crate::Reg<u32, _HUFFENC_AC0_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_1;
#[doc = "`read()` method returns [huffenc_ac0_1::R](huffenc_ac0_1::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_1 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_1::W](huffenc_ac0_1::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_1 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_1;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_2](huffenc_ac0_2) module"]
pub type HUFFENC_AC0_2 = crate::Reg<u32, _HUFFENC_AC0_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_2;
#[doc = "`read()` method returns [huffenc_ac0_2::R](huffenc_ac0_2::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_2 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_2::W](huffenc_ac0_2::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_2 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_2;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_3](huffenc_ac0_3) module"]
pub type HUFFENC_AC0_3 = crate::Reg<u32, _HUFFENC_AC0_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_3;
#[doc = "`read()` method returns [huffenc_ac0_3::R](huffenc_ac0_3::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_3 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_3::W](huffenc_ac0_3::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_3 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_3;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_4](huffenc_ac0_4) module"]
pub type HUFFENC_AC0_4 = crate::Reg<u32, _HUFFENC_AC0_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_4;
#[doc = "`read()` method returns [huffenc_ac0_4::R](huffenc_ac0_4::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_4 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_4::W](huffenc_ac0_4::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_4 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_4;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_5](huffenc_ac0_5) module"]
pub type HUFFENC_AC0_5 = crate::Reg<u32, _HUFFENC_AC0_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_5;
#[doc = "`read()` method returns [huffenc_ac0_5::R](huffenc_ac0_5::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_5 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_5::W](huffenc_ac0_5::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_5 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_5;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_6](huffenc_ac0_6) module"]
pub type HUFFENC_AC0_6 = crate::Reg<u32, _HUFFENC_AC0_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_6;
#[doc = "`read()` method returns [huffenc_ac0_6::R](huffenc_ac0_6::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_6 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_6::W](huffenc_ac0_6::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_6 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_6;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_7](huffenc_ac0_7) module"]
pub type HUFFENC_AC0_7 = crate::Reg<u32, _HUFFENC_AC0_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_7;
#[doc = "`read()` method returns [huffenc_ac0_7::R](huffenc_ac0_7::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_7 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_7::W](huffenc_ac0_7::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_7 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_7;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_8](huffenc_ac0_8) module"]
pub type HUFFENC_AC0_8 = crate::Reg<u32, _HUFFENC_AC0_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_8;
#[doc = "`read()` method returns [huffenc_ac0_8::R](huffenc_ac0_8::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_8 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_8::W](huffenc_ac0_8::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_8 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_8;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_9](huffenc_ac0_9) module"]
pub type HUFFENC_AC0_9 = crate::Reg<u32, _HUFFENC_AC0_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_9;
#[doc = "`read()` method returns [huffenc_ac0_9::R](huffenc_ac0_9::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_9 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_9::W](huffenc_ac0_9::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_9 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_9;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_10](huffenc_ac0_10) module"]
pub type HUFFENC_AC0_10 = crate::Reg<u32, _HUFFENC_AC0_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_10;
#[doc = "`read()` method returns [huffenc_ac0_10::R](huffenc_ac0_10::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_10 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_10::W](huffenc_ac0_10::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_10 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_10;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_11](huffenc_ac0_11) module"]
pub type HUFFENC_AC0_11 = crate::Reg<u32, _HUFFENC_AC0_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_11;
#[doc = "`read()` method returns [huffenc_ac0_11::R](huffenc_ac0_11::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_11 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_11::W](huffenc_ac0_11::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_11 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_11;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_12](huffenc_ac0_12) module"]
pub type HUFFENC_AC0_12 = crate::Reg<u32, _HUFFENC_AC0_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_12;
#[doc = "`read()` method returns [huffenc_ac0_12::R](huffenc_ac0_12::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_12 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_12::W](huffenc_ac0_12::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_12 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_12;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_13](huffenc_ac0_13) module"]
pub type HUFFENC_AC0_13 = crate::Reg<u32, _HUFFENC_AC0_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_13;
#[doc = "`read()` method returns [huffenc_ac0_13::R](huffenc_ac0_13::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_13 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_13::W](huffenc_ac0_13::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_13 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_13;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_14](huffenc_ac0_14) module"]
pub type HUFFENC_AC0_14 = crate::Reg<u32, _HUFFENC_AC0_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_14;
#[doc = "`read()` method returns [huffenc_ac0_14::R](huffenc_ac0_14::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_14 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_14::W](huffenc_ac0_14::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_14 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_14;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_15](huffenc_ac0_15) module"]
pub type HUFFENC_AC0_15 = crate::Reg<u32, _HUFFENC_AC0_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_15;
#[doc = "`read()` method returns [huffenc_ac0_15::R](huffenc_ac0_15::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_15 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_15::W](huffenc_ac0_15::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_15 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_15;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_16](huffenc_ac0_16) module"]
pub type HUFFENC_AC0_16 = crate::Reg<u32, _HUFFENC_AC0_16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_16;
#[doc = "`read()` method returns [huffenc_ac0_16::R](huffenc_ac0_16::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_16 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_16::W](huffenc_ac0_16::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_16 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_16;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_17](huffenc_ac0_17) module"]
pub type HUFFENC_AC0_17 = crate::Reg<u32, _HUFFENC_AC0_17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_17;
#[doc = "`read()` method returns [huffenc_ac0_17::R](huffenc_ac0_17::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_17 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_17::W](huffenc_ac0_17::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_17 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_17;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_18](huffenc_ac0_18) module"]
pub type HUFFENC_AC0_18 = crate::Reg<u32, _HUFFENC_AC0_18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_18;
#[doc = "`read()` method returns [huffenc_ac0_18::R](huffenc_ac0_18::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_18 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_18::W](huffenc_ac0_18::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_18 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_18;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_19](huffenc_ac0_19) module"]
pub type HUFFENC_AC0_19 = crate::Reg<u32, _HUFFENC_AC0_19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_19;
#[doc = "`read()` method returns [huffenc_ac0_19::R](huffenc_ac0_19::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_19 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_19::W](huffenc_ac0_19::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_19 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_19;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_20](huffenc_ac0_20) module"]
pub type HUFFENC_AC0_20 = crate::Reg<u32, _HUFFENC_AC0_20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_20;
#[doc = "`read()` method returns [huffenc_ac0_20::R](huffenc_ac0_20::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_20 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_20::W](huffenc_ac0_20::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_20 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_20;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_21](huffenc_ac0_21) module"]
pub type HUFFENC_AC0_21 = crate::Reg<u32, _HUFFENC_AC0_21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_21;
#[doc = "`read()` method returns [huffenc_ac0_21::R](huffenc_ac0_21::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_21 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_21::W](huffenc_ac0_21::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_21 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_21;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_22](huffenc_ac0_22) module"]
pub type HUFFENC_AC0_22 = crate::Reg<u32, _HUFFENC_AC0_22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_22;
#[doc = "`read()` method returns [huffenc_ac0_22::R](huffenc_ac0_22::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_22 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_22::W](huffenc_ac0_22::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_22 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_22;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_23](huffenc_ac0_23) module"]
pub type HUFFENC_AC0_23 = crate::Reg<u32, _HUFFENC_AC0_23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_23;
#[doc = "`read()` method returns [huffenc_ac0_23::R](huffenc_ac0_23::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_23 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_23::W](huffenc_ac0_23::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_23 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_23;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_24](huffenc_ac0_24) module"]
pub type HUFFENC_AC0_24 = crate::Reg<u32, _HUFFENC_AC0_24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_24;
#[doc = "`read()` method returns [huffenc_ac0_24::R](huffenc_ac0_24::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_24 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_24::W](huffenc_ac0_24::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_24 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_24;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_25](huffenc_ac0_25) module"]
pub type HUFFENC_AC0_25 = crate::Reg<u32, _HUFFENC_AC0_25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_25;
#[doc = "`read()` method returns [huffenc_ac0_25::R](huffenc_ac0_25::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_25 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_25::W](huffenc_ac0_25::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_25 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_25;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_26](huffenc_ac0_26) module"]
pub type HUFFENC_AC0_26 = crate::Reg<u32, _HUFFENC_AC0_26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_26;
#[doc = "`read()` method returns [huffenc_ac0_26::R](huffenc_ac0_26::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_26 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_26::W](huffenc_ac0_26::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_26 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_26;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_27](huffenc_ac0_27) module"]
pub type HUFFENC_AC0_27 = crate::Reg<u32, _HUFFENC_AC0_27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_27;
#[doc = "`read()` method returns [huffenc_ac0_27::R](huffenc_ac0_27::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_27 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_27::W](huffenc_ac0_27::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_27 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_27;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_28](huffenc_ac0_28) module"]
pub type HUFFENC_AC0_28 = crate::Reg<u32, _HUFFENC_AC0_28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_28;
#[doc = "`read()` method returns [huffenc_ac0_28::R](huffenc_ac0_28::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_28 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_28::W](huffenc_ac0_28::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_28 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_28;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_29](huffenc_ac0_29) module"]
pub type HUFFENC_AC0_29 = crate::Reg<u32, _HUFFENC_AC0_29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_29;
#[doc = "`read()` method returns [huffenc_ac0_29::R](huffenc_ac0_29::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_29 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_29::W](huffenc_ac0_29::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_29 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_29;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_30](huffenc_ac0_30) module"]
pub type HUFFENC_AC0_30 = crate::Reg<u32, _HUFFENC_AC0_30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_30;
#[doc = "`read()` method returns [huffenc_ac0_30::R](huffenc_ac0_30::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_30 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_30::W](huffenc_ac0_30::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_30 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_30;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_31](huffenc_ac0_31) module"]
pub type HUFFENC_AC0_31 = crate::Reg<u32, _HUFFENC_AC0_31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_31;
#[doc = "`read()` method returns [huffenc_ac0_31::R](huffenc_ac0_31::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_31 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_31::W](huffenc_ac0_31::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_31 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_31;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_32](huffenc_ac0_32) module"]
pub type HUFFENC_AC0_32 = crate::Reg<u32, _HUFFENC_AC0_32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_32;
#[doc = "`read()` method returns [huffenc_ac0_32::R](huffenc_ac0_32::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_32 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_32::W](huffenc_ac0_32::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_32 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_32;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_33](huffenc_ac0_33) module"]
pub type HUFFENC_AC0_33 = crate::Reg<u32, _HUFFENC_AC0_33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_33;
#[doc = "`read()` method returns [huffenc_ac0_33::R](huffenc_ac0_33::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_33 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_33::W](huffenc_ac0_33::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_33 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_33;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_34](huffenc_ac0_34) module"]
pub type HUFFENC_AC0_34 = crate::Reg<u32, _HUFFENC_AC0_34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_34;
#[doc = "`read()` method returns [huffenc_ac0_34::R](huffenc_ac0_34::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_34 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_34::W](huffenc_ac0_34::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_34 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_34;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_35](huffenc_ac0_35) module"]
pub type HUFFENC_AC0_35 = crate::Reg<u32, _HUFFENC_AC0_35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_35;
#[doc = "`read()` method returns [huffenc_ac0_35::R](huffenc_ac0_35::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_35 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_35::W](huffenc_ac0_35::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_35 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_35;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_36](huffenc_ac0_36) module"]
pub type HUFFENC_AC0_36 = crate::Reg<u32, _HUFFENC_AC0_36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_36;
#[doc = "`read()` method returns [huffenc_ac0_36::R](huffenc_ac0_36::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_36 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_36::W](huffenc_ac0_36::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_36 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_36;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_37](huffenc_ac0_37) module"]
pub type HUFFENC_AC0_37 = crate::Reg<u32, _HUFFENC_AC0_37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_37;
#[doc = "`read()` method returns [huffenc_ac0_37::R](huffenc_ac0_37::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_37 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_37::W](huffenc_ac0_37::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_37 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_37;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_38](huffenc_ac0_38) module"]
pub type HUFFENC_AC0_38 = crate::Reg<u32, _HUFFENC_AC0_38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_38;
#[doc = "`read()` method returns [huffenc_ac0_38::R](huffenc_ac0_38::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_38 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_38::W](huffenc_ac0_38::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_38 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_38;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_39](huffenc_ac0_39) module"]
pub type HUFFENC_AC0_39 = crate::Reg<u32, _HUFFENC_AC0_39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_39;
#[doc = "`read()` method returns [huffenc_ac0_39::R](huffenc_ac0_39::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_39 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_39::W](huffenc_ac0_39::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_39 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_39;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_40](huffenc_ac0_40) module"]
pub type HUFFENC_AC0_40 = crate::Reg<u32, _HUFFENC_AC0_40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_40;
#[doc = "`read()` method returns [huffenc_ac0_40::R](huffenc_ac0_40::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_40 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_40::W](huffenc_ac0_40::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_40 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_40;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_41](huffenc_ac0_41) module"]
pub type HUFFENC_AC0_41 = crate::Reg<u32, _HUFFENC_AC0_41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_41;
#[doc = "`read()` method returns [huffenc_ac0_41::R](huffenc_ac0_41::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_41 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_41::W](huffenc_ac0_41::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_41 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_41;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_42](huffenc_ac0_42) module"]
pub type HUFFENC_AC0_42 = crate::Reg<u32, _HUFFENC_AC0_42>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_42;
#[doc = "`read()` method returns [huffenc_ac0_42::R](huffenc_ac0_42::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_42 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_42::W](huffenc_ac0_42::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_42 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_42;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_43](huffenc_ac0_43) module"]
pub type HUFFENC_AC0_43 = crate::Reg<u32, _HUFFENC_AC0_43>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_43;
#[doc = "`read()` method returns [huffenc_ac0_43::R](huffenc_ac0_43::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_43 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_43::W](huffenc_ac0_43::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_43 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_43;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_44](huffenc_ac0_44) module"]
pub type HUFFENC_AC0_44 = crate::Reg<u32, _HUFFENC_AC0_44>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_44;
#[doc = "`read()` method returns [huffenc_ac0_44::R](huffenc_ac0_44::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_44 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_44::W](huffenc_ac0_44::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_44 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_44;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_45](huffenc_ac0_45) module"]
pub type HUFFENC_AC0_45 = crate::Reg<u32, _HUFFENC_AC0_45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_45;
#[doc = "`read()` method returns [huffenc_ac0_45::R](huffenc_ac0_45::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_45 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_45::W](huffenc_ac0_45::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_45 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_45;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_46](huffenc_ac0_46) module"]
pub type HUFFENC_AC0_46 = crate::Reg<u32, _HUFFENC_AC0_46>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_46;
#[doc = "`read()` method returns [huffenc_ac0_46::R](huffenc_ac0_46::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_46 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_46::W](huffenc_ac0_46::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_46 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_46;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_47](huffenc_ac0_47) module"]
pub type HUFFENC_AC0_47 = crate::Reg<u32, _HUFFENC_AC0_47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_47;
#[doc = "`read()` method returns [huffenc_ac0_47::R](huffenc_ac0_47::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_47 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_47::W](huffenc_ac0_47::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_47 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_47;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_48](huffenc_ac0_48) module"]
pub type HUFFENC_AC0_48 = crate::Reg<u32, _HUFFENC_AC0_48>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_48;
#[doc = "`read()` method returns [huffenc_ac0_48::R](huffenc_ac0_48::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_48 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_48::W](huffenc_ac0_48::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_48 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_48;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_49](huffenc_ac0_49) module"]
pub type HUFFENC_AC0_49 = crate::Reg<u32, _HUFFENC_AC0_49>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_49;
#[doc = "`read()` method returns [huffenc_ac0_49::R](huffenc_ac0_49::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_49 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_49::W](huffenc_ac0_49::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_49 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_49;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_50](huffenc_ac0_50) module"]
pub type HUFFENC_AC0_50 = crate::Reg<u32, _HUFFENC_AC0_50>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_50;
#[doc = "`read()` method returns [huffenc_ac0_50::R](huffenc_ac0_50::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_50 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_50::W](huffenc_ac0_50::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_50 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_50;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_51](huffenc_ac0_51) module"]
pub type HUFFENC_AC0_51 = crate::Reg<u32, _HUFFENC_AC0_51>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_51;
#[doc = "`read()` method returns [huffenc_ac0_51::R](huffenc_ac0_51::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_51 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_51::W](huffenc_ac0_51::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_51 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_51;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_52](huffenc_ac0_52) module"]
pub type HUFFENC_AC0_52 = crate::Reg<u32, _HUFFENC_AC0_52>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_52;
#[doc = "`read()` method returns [huffenc_ac0_52::R](huffenc_ac0_52::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_52 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_52::W](huffenc_ac0_52::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_52 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_52;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_53](huffenc_ac0_53) module"]
pub type HUFFENC_AC0_53 = crate::Reg<u32, _HUFFENC_AC0_53>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_53;
#[doc = "`read()` method returns [huffenc_ac0_53::R](huffenc_ac0_53::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_53 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_53::W](huffenc_ac0_53::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_53 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_53;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_54](huffenc_ac0_54) module"]
pub type HUFFENC_AC0_54 = crate::Reg<u32, _HUFFENC_AC0_54>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_54;
#[doc = "`read()` method returns [huffenc_ac0_54::R](huffenc_ac0_54::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_54 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_54::W](huffenc_ac0_54::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_54 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_54;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_55](huffenc_ac0_55) module"]
pub type HUFFENC_AC0_55 = crate::Reg<u32, _HUFFENC_AC0_55>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_55;
#[doc = "`read()` method returns [huffenc_ac0_55::R](huffenc_ac0_55::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_55 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_55::W](huffenc_ac0_55::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_55 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_55;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_56](huffenc_ac0_56) module"]
pub type HUFFENC_AC0_56 = crate::Reg<u32, _HUFFENC_AC0_56>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_56;
#[doc = "`read()` method returns [huffenc_ac0_56::R](huffenc_ac0_56::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_56 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_56::W](huffenc_ac0_56::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_56 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_56;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_57](huffenc_ac0_57) module"]
pub type HUFFENC_AC0_57 = crate::Reg<u32, _HUFFENC_AC0_57>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_57;
#[doc = "`read()` method returns [huffenc_ac0_57::R](huffenc_ac0_57::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_57 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_57::W](huffenc_ac0_57::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_57 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_57;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_58](huffenc_ac0_58) module"]
pub type HUFFENC_AC0_58 = crate::Reg<u32, _HUFFENC_AC0_58>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_58;
#[doc = "`read()` method returns [huffenc_ac0_58::R](huffenc_ac0_58::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_58 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_58::W](huffenc_ac0_58::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_58 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_58;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_59](huffenc_ac0_59) module"]
pub type HUFFENC_AC0_59 = crate::Reg<u32, _HUFFENC_AC0_59>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_59;
#[doc = "`read()` method returns [huffenc_ac0_59::R](huffenc_ac0_59::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_59 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_59::W](huffenc_ac0_59::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_59 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_59;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_60](huffenc_ac0_60) module"]
pub type HUFFENC_AC0_60 = crate::Reg<u32, _HUFFENC_AC0_60>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_60;
#[doc = "`read()` method returns [huffenc_ac0_60::R](huffenc_ac0_60::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_60 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_60::W](huffenc_ac0_60::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_60 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_60;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_61](huffenc_ac0_61) module"]
pub type HUFFENC_AC0_61 = crate::Reg<u32, _HUFFENC_AC0_61>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_61;
#[doc = "`read()` method returns [huffenc_ac0_61::R](huffenc_ac0_61::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_61 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_61::W](huffenc_ac0_61::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_61 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_61;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_62](huffenc_ac0_62) module"]
pub type HUFFENC_AC0_62 = crate::Reg<u32, _HUFFENC_AC0_62>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_62;
#[doc = "`read()` method returns [huffenc_ac0_62::R](huffenc_ac0_62::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_62 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_62::W](huffenc_ac0_62::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_62 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_62;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_63](huffenc_ac0_63) module"]
pub type HUFFENC_AC0_63 = crate::Reg<u32, _HUFFENC_AC0_63>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_63;
#[doc = "`read()` method returns [huffenc_ac0_63::R](huffenc_ac0_63::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_63 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_63::W](huffenc_ac0_63::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_63 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_63;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_64](huffenc_ac0_64) module"]
pub type HUFFENC_AC0_64 = crate::Reg<u32, _HUFFENC_AC0_64>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_64;
#[doc = "`read()` method returns [huffenc_ac0_64::R](huffenc_ac0_64::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_64 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_64::W](huffenc_ac0_64::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_64 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_64;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_65](huffenc_ac0_65) module"]
pub type HUFFENC_AC0_65 = crate::Reg<u32, _HUFFENC_AC0_65>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_65;
#[doc = "`read()` method returns [huffenc_ac0_65::R](huffenc_ac0_65::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_65 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_65::W](huffenc_ac0_65::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_65 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_65;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_66](huffenc_ac0_66) module"]
pub type HUFFENC_AC0_66 = crate::Reg<u32, _HUFFENC_AC0_66>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_66;
#[doc = "`read()` method returns [huffenc_ac0_66::R](huffenc_ac0_66::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_66 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_66::W](huffenc_ac0_66::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_66 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_66;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_67](huffenc_ac0_67) module"]
pub type HUFFENC_AC0_67 = crate::Reg<u32, _HUFFENC_AC0_67>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_67;
#[doc = "`read()` method returns [huffenc_ac0_67::R](huffenc_ac0_67::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_67 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_67::W](huffenc_ac0_67::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_67 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_67;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_68](huffenc_ac0_68) module"]
pub type HUFFENC_AC0_68 = crate::Reg<u32, _HUFFENC_AC0_68>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_68;
#[doc = "`read()` method returns [huffenc_ac0_68::R](huffenc_ac0_68::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_68 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_68::W](huffenc_ac0_68::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_68 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_68;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_69](huffenc_ac0_69) module"]
pub type HUFFENC_AC0_69 = crate::Reg<u32, _HUFFENC_AC0_69>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_69;
#[doc = "`read()` method returns [huffenc_ac0_69::R](huffenc_ac0_69::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_69 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_69::W](huffenc_ac0_69::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_69 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_69;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_70](huffenc_ac0_70) module"]
pub type HUFFENC_AC0_70 = crate::Reg<u32, _HUFFENC_AC0_70>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_70;
#[doc = "`read()` method returns [huffenc_ac0_70::R](huffenc_ac0_70::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_70 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_70::W](huffenc_ac0_70::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_70 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_70;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_71](huffenc_ac0_71) module"]
pub type HUFFENC_AC0_71 = crate::Reg<u32, _HUFFENC_AC0_71>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_71;
#[doc = "`read()` method returns [huffenc_ac0_71::R](huffenc_ac0_71::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_71 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_71::W](huffenc_ac0_71::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_71 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_71;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_72](huffenc_ac0_72) module"]
pub type HUFFENC_AC0_72 = crate::Reg<u32, _HUFFENC_AC0_72>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_72;
#[doc = "`read()` method returns [huffenc_ac0_72::R](huffenc_ac0_72::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_72 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_72::W](huffenc_ac0_72::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_72 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_72;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_73](huffenc_ac0_73) module"]
pub type HUFFENC_AC0_73 = crate::Reg<u32, _HUFFENC_AC0_73>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_73;
#[doc = "`read()` method returns [huffenc_ac0_73::R](huffenc_ac0_73::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_73 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_73::W](huffenc_ac0_73::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_73 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_73;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_74](huffenc_ac0_74) module"]
pub type HUFFENC_AC0_74 = crate::Reg<u32, _HUFFENC_AC0_74>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_74;
#[doc = "`read()` method returns [huffenc_ac0_74::R](huffenc_ac0_74::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_74 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_74::W](huffenc_ac0_74::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_74 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_74;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_75](huffenc_ac0_75) module"]
pub type HUFFENC_AC0_75 = crate::Reg<u32, _HUFFENC_AC0_75>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_75;
#[doc = "`read()` method returns [huffenc_ac0_75::R](huffenc_ac0_75::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_75 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_75::W](huffenc_ac0_75::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_75 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_75;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_76](huffenc_ac0_76) module"]
pub type HUFFENC_AC0_76 = crate::Reg<u32, _HUFFENC_AC0_76>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_76;
#[doc = "`read()` method returns [huffenc_ac0_76::R](huffenc_ac0_76::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_76 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_76::W](huffenc_ac0_76::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_76 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_76;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_77](huffenc_ac0_77) module"]
pub type HUFFENC_AC0_77 = crate::Reg<u32, _HUFFENC_AC0_77>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_77;
#[doc = "`read()` method returns [huffenc_ac0_77::R](huffenc_ac0_77::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_77 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_77::W](huffenc_ac0_77::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_77 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_77;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_78](huffenc_ac0_78) module"]
pub type HUFFENC_AC0_78 = crate::Reg<u32, _HUFFENC_AC0_78>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_78;
#[doc = "`read()` method returns [huffenc_ac0_78::R](huffenc_ac0_78::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_78 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_78::W](huffenc_ac0_78::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_78 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_78;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_79](huffenc_ac0_79) module"]
pub type HUFFENC_AC0_79 = crate::Reg<u32, _HUFFENC_AC0_79>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_79;
#[doc = "`read()` method returns [huffenc_ac0_79::R](huffenc_ac0_79::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_79 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_79::W](huffenc_ac0_79::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_79 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_79;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_80](huffenc_ac0_80) module"]
pub type HUFFENC_AC0_80 = crate::Reg<u32, _HUFFENC_AC0_80>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_80;
#[doc = "`read()` method returns [huffenc_ac0_80::R](huffenc_ac0_80::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_80 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_80::W](huffenc_ac0_80::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_80 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_80;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_81](huffenc_ac0_81) module"]
pub type HUFFENC_AC0_81 = crate::Reg<u32, _HUFFENC_AC0_81>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_81;
#[doc = "`read()` method returns [huffenc_ac0_81::R](huffenc_ac0_81::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_81 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_81::W](huffenc_ac0_81::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_81 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_81;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_82](huffenc_ac0_82) module"]
pub type HUFFENC_AC0_82 = crate::Reg<u32, _HUFFENC_AC0_82>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_82;
#[doc = "`read()` method returns [huffenc_ac0_82::R](huffenc_ac0_82::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_82 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_82::W](huffenc_ac0_82::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_82 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_82;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_83](huffenc_ac0_83) module"]
pub type HUFFENC_AC0_83 = crate::Reg<u32, _HUFFENC_AC0_83>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_83;
#[doc = "`read()` method returns [huffenc_ac0_83::R](huffenc_ac0_83::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_83 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_83::W](huffenc_ac0_83::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_83 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_83;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_84](huffenc_ac0_84) module"]
pub type HUFFENC_AC0_84 = crate::Reg<u32, _HUFFENC_AC0_84>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_84;
#[doc = "`read()` method returns [huffenc_ac0_84::R](huffenc_ac0_84::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_84 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_84::W](huffenc_ac0_84::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_84 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_84;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_85](huffenc_ac0_85) module"]
pub type HUFFENC_AC0_85 = crate::Reg<u32, _HUFFENC_AC0_85>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_85;
#[doc = "`read()` method returns [huffenc_ac0_85::R](huffenc_ac0_85::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_85 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_85::W](huffenc_ac0_85::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_85 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_85;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_86](huffenc_ac0_86) module"]
pub type HUFFENC_AC0_86 = crate::Reg<u32, _HUFFENC_AC0_86>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_86;
#[doc = "`read()` method returns [huffenc_ac0_86::R](huffenc_ac0_86::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_86 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_86::W](huffenc_ac0_86::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_86 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_86;
#[doc = "JPEG encoder, AC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac0_87](huffenc_ac0_87) module"]
pub type HUFFENC_AC0_87 = crate::Reg<u32, _HUFFENC_AC0_87>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC0_87;
#[doc = "`read()` method returns [huffenc_ac0_87::R](huffenc_ac0_87::R) reader structure"]
impl crate::Readable for HUFFENC_AC0_87 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac0_87::W](huffenc_ac0_87::W) writer structure"]
impl crate::Writable for HUFFENC_AC0_87 {}
#[doc = "JPEG encoder, AC Huffman table 0"]
pub mod huffenc_ac0_87;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_0](huffenc_ac1_0) module"]
pub type HUFFENC_AC1_0 = crate::Reg<u32, _HUFFENC_AC1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_0;
#[doc = "`read()` method returns [huffenc_ac1_0::R](huffenc_ac1_0::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_0 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_0::W](huffenc_ac1_0::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_0 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_0;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_1](huffenc_ac1_1) module"]
pub type HUFFENC_AC1_1 = crate::Reg<u32, _HUFFENC_AC1_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_1;
#[doc = "`read()` method returns [huffenc_ac1_1::R](huffenc_ac1_1::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_1 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_1::W](huffenc_ac1_1::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_1 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_1;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_2](huffenc_ac1_2) module"]
pub type HUFFENC_AC1_2 = crate::Reg<u32, _HUFFENC_AC1_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_2;
#[doc = "`read()` method returns [huffenc_ac1_2::R](huffenc_ac1_2::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_2 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_2::W](huffenc_ac1_2::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_2 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_2;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_3](huffenc_ac1_3) module"]
pub type HUFFENC_AC1_3 = crate::Reg<u32, _HUFFENC_AC1_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_3;
#[doc = "`read()` method returns [huffenc_ac1_3::R](huffenc_ac1_3::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_3 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_3::W](huffenc_ac1_3::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_3 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_3;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_4](huffenc_ac1_4) module"]
pub type HUFFENC_AC1_4 = crate::Reg<u32, _HUFFENC_AC1_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_4;
#[doc = "`read()` method returns [huffenc_ac1_4::R](huffenc_ac1_4::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_4 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_4::W](huffenc_ac1_4::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_4 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_4;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_5](huffenc_ac1_5) module"]
pub type HUFFENC_AC1_5 = crate::Reg<u32, _HUFFENC_AC1_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_5;
#[doc = "`read()` method returns [huffenc_ac1_5::R](huffenc_ac1_5::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_5 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_5::W](huffenc_ac1_5::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_5 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_5;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_6](huffenc_ac1_6) module"]
pub type HUFFENC_AC1_6 = crate::Reg<u32, _HUFFENC_AC1_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_6;
#[doc = "`read()` method returns [huffenc_ac1_6::R](huffenc_ac1_6::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_6 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_6::W](huffenc_ac1_6::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_6 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_6;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_7](huffenc_ac1_7) module"]
pub type HUFFENC_AC1_7 = crate::Reg<u32, _HUFFENC_AC1_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_7;
#[doc = "`read()` method returns [huffenc_ac1_7::R](huffenc_ac1_7::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_7 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_7::W](huffenc_ac1_7::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_7 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_7;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_8](huffenc_ac1_8) module"]
pub type HUFFENC_AC1_8 = crate::Reg<u32, _HUFFENC_AC1_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_8;
#[doc = "`read()` method returns [huffenc_ac1_8::R](huffenc_ac1_8::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_8 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_8::W](huffenc_ac1_8::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_8 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_8;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_9](huffenc_ac1_9) module"]
pub type HUFFENC_AC1_9 = crate::Reg<u32, _HUFFENC_AC1_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_9;
#[doc = "`read()` method returns [huffenc_ac1_9::R](huffenc_ac1_9::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_9 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_9::W](huffenc_ac1_9::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_9 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_9;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_10](huffenc_ac1_10) module"]
pub type HUFFENC_AC1_10 = crate::Reg<u32, _HUFFENC_AC1_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_10;
#[doc = "`read()` method returns [huffenc_ac1_10::R](huffenc_ac1_10::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_10 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_10::W](huffenc_ac1_10::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_10 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_10;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_11](huffenc_ac1_11) module"]
pub type HUFFENC_AC1_11 = crate::Reg<u32, _HUFFENC_AC1_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_11;
#[doc = "`read()` method returns [huffenc_ac1_11::R](huffenc_ac1_11::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_11 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_11::W](huffenc_ac1_11::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_11 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_11;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_12](huffenc_ac1_12) module"]
pub type HUFFENC_AC1_12 = crate::Reg<u32, _HUFFENC_AC1_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_12;
#[doc = "`read()` method returns [huffenc_ac1_12::R](huffenc_ac1_12::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_12 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_12::W](huffenc_ac1_12::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_12 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_12;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_13](huffenc_ac1_13) module"]
pub type HUFFENC_AC1_13 = crate::Reg<u32, _HUFFENC_AC1_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_13;
#[doc = "`read()` method returns [huffenc_ac1_13::R](huffenc_ac1_13::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_13 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_13::W](huffenc_ac1_13::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_13 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_13;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_14](huffenc_ac1_14) module"]
pub type HUFFENC_AC1_14 = crate::Reg<u32, _HUFFENC_AC1_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_14;
#[doc = "`read()` method returns [huffenc_ac1_14::R](huffenc_ac1_14::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_14 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_14::W](huffenc_ac1_14::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_14 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_14;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_15](huffenc_ac1_15) module"]
pub type HUFFENC_AC1_15 = crate::Reg<u32, _HUFFENC_AC1_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_15;
#[doc = "`read()` method returns [huffenc_ac1_15::R](huffenc_ac1_15::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_15 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_15::W](huffenc_ac1_15::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_15 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_15;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_16](huffenc_ac1_16) module"]
pub type HUFFENC_AC1_16 = crate::Reg<u32, _HUFFENC_AC1_16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_16;
#[doc = "`read()` method returns [huffenc_ac1_16::R](huffenc_ac1_16::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_16 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_16::W](huffenc_ac1_16::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_16 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_16;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_17](huffenc_ac1_17) module"]
pub type HUFFENC_AC1_17 = crate::Reg<u32, _HUFFENC_AC1_17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_17;
#[doc = "`read()` method returns [huffenc_ac1_17::R](huffenc_ac1_17::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_17 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_17::W](huffenc_ac1_17::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_17 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_17;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_18](huffenc_ac1_18) module"]
pub type HUFFENC_AC1_18 = crate::Reg<u32, _HUFFENC_AC1_18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_18;
#[doc = "`read()` method returns [huffenc_ac1_18::R](huffenc_ac1_18::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_18 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_18::W](huffenc_ac1_18::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_18 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_18;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_19](huffenc_ac1_19) module"]
pub type HUFFENC_AC1_19 = crate::Reg<u32, _HUFFENC_AC1_19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_19;
#[doc = "`read()` method returns [huffenc_ac1_19::R](huffenc_ac1_19::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_19 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_19::W](huffenc_ac1_19::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_19 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_19;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_20](huffenc_ac1_20) module"]
pub type HUFFENC_AC1_20 = crate::Reg<u32, _HUFFENC_AC1_20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_20;
#[doc = "`read()` method returns [huffenc_ac1_20::R](huffenc_ac1_20::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_20 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_20::W](huffenc_ac1_20::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_20 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_20;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_21](huffenc_ac1_21) module"]
pub type HUFFENC_AC1_21 = crate::Reg<u32, _HUFFENC_AC1_21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_21;
#[doc = "`read()` method returns [huffenc_ac1_21::R](huffenc_ac1_21::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_21 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_21::W](huffenc_ac1_21::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_21 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_21;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_22](huffenc_ac1_22) module"]
pub type HUFFENC_AC1_22 = crate::Reg<u32, _HUFFENC_AC1_22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_22;
#[doc = "`read()` method returns [huffenc_ac1_22::R](huffenc_ac1_22::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_22 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_22::W](huffenc_ac1_22::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_22 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_22;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_23](huffenc_ac1_23) module"]
pub type HUFFENC_AC1_23 = crate::Reg<u32, _HUFFENC_AC1_23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_23;
#[doc = "`read()` method returns [huffenc_ac1_23::R](huffenc_ac1_23::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_23 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_23::W](huffenc_ac1_23::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_23 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_23;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_24](huffenc_ac1_24) module"]
pub type HUFFENC_AC1_24 = crate::Reg<u32, _HUFFENC_AC1_24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_24;
#[doc = "`read()` method returns [huffenc_ac1_24::R](huffenc_ac1_24::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_24 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_24::W](huffenc_ac1_24::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_24 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_24;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_25](huffenc_ac1_25) module"]
pub type HUFFENC_AC1_25 = crate::Reg<u32, _HUFFENC_AC1_25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_25;
#[doc = "`read()` method returns [huffenc_ac1_25::R](huffenc_ac1_25::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_25 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_25::W](huffenc_ac1_25::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_25 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_25;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_26](huffenc_ac1_26) module"]
pub type HUFFENC_AC1_26 = crate::Reg<u32, _HUFFENC_AC1_26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_26;
#[doc = "`read()` method returns [huffenc_ac1_26::R](huffenc_ac1_26::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_26 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_26::W](huffenc_ac1_26::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_26 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_26;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_27](huffenc_ac1_27) module"]
pub type HUFFENC_AC1_27 = crate::Reg<u32, _HUFFENC_AC1_27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_27;
#[doc = "`read()` method returns [huffenc_ac1_27::R](huffenc_ac1_27::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_27 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_27::W](huffenc_ac1_27::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_27 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_27;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_28](huffenc_ac1_28) module"]
pub type HUFFENC_AC1_28 = crate::Reg<u32, _HUFFENC_AC1_28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_28;
#[doc = "`read()` method returns [huffenc_ac1_28::R](huffenc_ac1_28::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_28 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_28::W](huffenc_ac1_28::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_28 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_28;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_29](huffenc_ac1_29) module"]
pub type HUFFENC_AC1_29 = crate::Reg<u32, _HUFFENC_AC1_29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_29;
#[doc = "`read()` method returns [huffenc_ac1_29::R](huffenc_ac1_29::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_29 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_29::W](huffenc_ac1_29::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_29 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_29;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_30](huffenc_ac1_30) module"]
pub type HUFFENC_AC1_30 = crate::Reg<u32, _HUFFENC_AC1_30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_30;
#[doc = "`read()` method returns [huffenc_ac1_30::R](huffenc_ac1_30::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_30 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_30::W](huffenc_ac1_30::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_30 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_30;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_31](huffenc_ac1_31) module"]
pub type HUFFENC_AC1_31 = crate::Reg<u32, _HUFFENC_AC1_31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_31;
#[doc = "`read()` method returns [huffenc_ac1_31::R](huffenc_ac1_31::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_31 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_31::W](huffenc_ac1_31::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_31 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_31;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_32](huffenc_ac1_32) module"]
pub type HUFFENC_AC1_32 = crate::Reg<u32, _HUFFENC_AC1_32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_32;
#[doc = "`read()` method returns [huffenc_ac1_32::R](huffenc_ac1_32::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_32 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_32::W](huffenc_ac1_32::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_32 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_32;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_33](huffenc_ac1_33) module"]
pub type HUFFENC_AC1_33 = crate::Reg<u32, _HUFFENC_AC1_33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_33;
#[doc = "`read()` method returns [huffenc_ac1_33::R](huffenc_ac1_33::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_33 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_33::W](huffenc_ac1_33::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_33 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_33;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_34](huffenc_ac1_34) module"]
pub type HUFFENC_AC1_34 = crate::Reg<u32, _HUFFENC_AC1_34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_34;
#[doc = "`read()` method returns [huffenc_ac1_34::R](huffenc_ac1_34::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_34 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_34::W](huffenc_ac1_34::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_34 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_34;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_35](huffenc_ac1_35) module"]
pub type HUFFENC_AC1_35 = crate::Reg<u32, _HUFFENC_AC1_35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_35;
#[doc = "`read()` method returns [huffenc_ac1_35::R](huffenc_ac1_35::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_35 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_35::W](huffenc_ac1_35::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_35 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_35;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_36](huffenc_ac1_36) module"]
pub type HUFFENC_AC1_36 = crate::Reg<u32, _HUFFENC_AC1_36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_36;
#[doc = "`read()` method returns [huffenc_ac1_36::R](huffenc_ac1_36::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_36 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_36::W](huffenc_ac1_36::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_36 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_36;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_37](huffenc_ac1_37) module"]
pub type HUFFENC_AC1_37 = crate::Reg<u32, _HUFFENC_AC1_37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_37;
#[doc = "`read()` method returns [huffenc_ac1_37::R](huffenc_ac1_37::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_37 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_37::W](huffenc_ac1_37::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_37 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_37;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_38](huffenc_ac1_38) module"]
pub type HUFFENC_AC1_38 = crate::Reg<u32, _HUFFENC_AC1_38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_38;
#[doc = "`read()` method returns [huffenc_ac1_38::R](huffenc_ac1_38::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_38 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_38::W](huffenc_ac1_38::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_38 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_38;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_39](huffenc_ac1_39) module"]
pub type HUFFENC_AC1_39 = crate::Reg<u32, _HUFFENC_AC1_39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_39;
#[doc = "`read()` method returns [huffenc_ac1_39::R](huffenc_ac1_39::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_39 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_39::W](huffenc_ac1_39::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_39 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_39;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_40](huffenc_ac1_40) module"]
pub type HUFFENC_AC1_40 = crate::Reg<u32, _HUFFENC_AC1_40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_40;
#[doc = "`read()` method returns [huffenc_ac1_40::R](huffenc_ac1_40::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_40 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_40::W](huffenc_ac1_40::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_40 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_40;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_41](huffenc_ac1_41) module"]
pub type HUFFENC_AC1_41 = crate::Reg<u32, _HUFFENC_AC1_41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_41;
#[doc = "`read()` method returns [huffenc_ac1_41::R](huffenc_ac1_41::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_41 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_41::W](huffenc_ac1_41::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_41 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_41;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_42](huffenc_ac1_42) module"]
pub type HUFFENC_AC1_42 = crate::Reg<u32, _HUFFENC_AC1_42>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_42;
#[doc = "`read()` method returns [huffenc_ac1_42::R](huffenc_ac1_42::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_42 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_42::W](huffenc_ac1_42::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_42 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_42;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_43](huffenc_ac1_43) module"]
pub type HUFFENC_AC1_43 = crate::Reg<u32, _HUFFENC_AC1_43>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_43;
#[doc = "`read()` method returns [huffenc_ac1_43::R](huffenc_ac1_43::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_43 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_43::W](huffenc_ac1_43::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_43 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_43;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_44](huffenc_ac1_44) module"]
pub type HUFFENC_AC1_44 = crate::Reg<u32, _HUFFENC_AC1_44>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_44;
#[doc = "`read()` method returns [huffenc_ac1_44::R](huffenc_ac1_44::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_44 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_44::W](huffenc_ac1_44::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_44 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_44;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_45](huffenc_ac1_45) module"]
pub type HUFFENC_AC1_45 = crate::Reg<u32, _HUFFENC_AC1_45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_45;
#[doc = "`read()` method returns [huffenc_ac1_45::R](huffenc_ac1_45::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_45 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_45::W](huffenc_ac1_45::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_45 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_45;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_46](huffenc_ac1_46) module"]
pub type HUFFENC_AC1_46 = crate::Reg<u32, _HUFFENC_AC1_46>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_46;
#[doc = "`read()` method returns [huffenc_ac1_46::R](huffenc_ac1_46::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_46 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_46::W](huffenc_ac1_46::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_46 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_46;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_47](huffenc_ac1_47) module"]
pub type HUFFENC_AC1_47 = crate::Reg<u32, _HUFFENC_AC1_47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_47;
#[doc = "`read()` method returns [huffenc_ac1_47::R](huffenc_ac1_47::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_47 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_47::W](huffenc_ac1_47::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_47 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_47;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_48](huffenc_ac1_48) module"]
pub type HUFFENC_AC1_48 = crate::Reg<u32, _HUFFENC_AC1_48>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_48;
#[doc = "`read()` method returns [huffenc_ac1_48::R](huffenc_ac1_48::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_48 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_48::W](huffenc_ac1_48::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_48 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_48;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_49](huffenc_ac1_49) module"]
pub type HUFFENC_AC1_49 = crate::Reg<u32, _HUFFENC_AC1_49>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_49;
#[doc = "`read()` method returns [huffenc_ac1_49::R](huffenc_ac1_49::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_49 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_49::W](huffenc_ac1_49::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_49 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_49;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_50](huffenc_ac1_50) module"]
pub type HUFFENC_AC1_50 = crate::Reg<u32, _HUFFENC_AC1_50>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_50;
#[doc = "`read()` method returns [huffenc_ac1_50::R](huffenc_ac1_50::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_50 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_50::W](huffenc_ac1_50::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_50 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_50;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_51](huffenc_ac1_51) module"]
pub type HUFFENC_AC1_51 = crate::Reg<u32, _HUFFENC_AC1_51>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_51;
#[doc = "`read()` method returns [huffenc_ac1_51::R](huffenc_ac1_51::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_51 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_51::W](huffenc_ac1_51::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_51 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_51;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_52](huffenc_ac1_52) module"]
pub type HUFFENC_AC1_52 = crate::Reg<u32, _HUFFENC_AC1_52>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_52;
#[doc = "`read()` method returns [huffenc_ac1_52::R](huffenc_ac1_52::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_52 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_52::W](huffenc_ac1_52::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_52 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_52;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_53](huffenc_ac1_53) module"]
pub type HUFFENC_AC1_53 = crate::Reg<u32, _HUFFENC_AC1_53>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_53;
#[doc = "`read()` method returns [huffenc_ac1_53::R](huffenc_ac1_53::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_53 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_53::W](huffenc_ac1_53::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_53 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_53;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_54](huffenc_ac1_54) module"]
pub type HUFFENC_AC1_54 = crate::Reg<u32, _HUFFENC_AC1_54>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_54;
#[doc = "`read()` method returns [huffenc_ac1_54::R](huffenc_ac1_54::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_54 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_54::W](huffenc_ac1_54::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_54 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_54;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_55](huffenc_ac1_55) module"]
pub type HUFFENC_AC1_55 = crate::Reg<u32, _HUFFENC_AC1_55>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_55;
#[doc = "`read()` method returns [huffenc_ac1_55::R](huffenc_ac1_55::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_55 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_55::W](huffenc_ac1_55::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_55 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_55;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_56](huffenc_ac1_56) module"]
pub type HUFFENC_AC1_56 = crate::Reg<u32, _HUFFENC_AC1_56>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_56;
#[doc = "`read()` method returns [huffenc_ac1_56::R](huffenc_ac1_56::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_56 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_56::W](huffenc_ac1_56::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_56 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_56;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_57](huffenc_ac1_57) module"]
pub type HUFFENC_AC1_57 = crate::Reg<u32, _HUFFENC_AC1_57>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_57;
#[doc = "`read()` method returns [huffenc_ac1_57::R](huffenc_ac1_57::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_57 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_57::W](huffenc_ac1_57::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_57 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_57;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_58](huffenc_ac1_58) module"]
pub type HUFFENC_AC1_58 = crate::Reg<u32, _HUFFENC_AC1_58>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_58;
#[doc = "`read()` method returns [huffenc_ac1_58::R](huffenc_ac1_58::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_58 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_58::W](huffenc_ac1_58::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_58 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_58;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_59](huffenc_ac1_59) module"]
pub type HUFFENC_AC1_59 = crate::Reg<u32, _HUFFENC_AC1_59>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_59;
#[doc = "`read()` method returns [huffenc_ac1_59::R](huffenc_ac1_59::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_59 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_59::W](huffenc_ac1_59::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_59 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_59;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_60](huffenc_ac1_60) module"]
pub type HUFFENC_AC1_60 = crate::Reg<u32, _HUFFENC_AC1_60>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_60;
#[doc = "`read()` method returns [huffenc_ac1_60::R](huffenc_ac1_60::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_60 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_60::W](huffenc_ac1_60::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_60 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_60;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_61](huffenc_ac1_61) module"]
pub type HUFFENC_AC1_61 = crate::Reg<u32, _HUFFENC_AC1_61>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_61;
#[doc = "`read()` method returns [huffenc_ac1_61::R](huffenc_ac1_61::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_61 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_61::W](huffenc_ac1_61::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_61 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_61;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_62](huffenc_ac1_62) module"]
pub type HUFFENC_AC1_62 = crate::Reg<u32, _HUFFENC_AC1_62>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_62;
#[doc = "`read()` method returns [huffenc_ac1_62::R](huffenc_ac1_62::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_62 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_62::W](huffenc_ac1_62::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_62 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_62;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_63](huffenc_ac1_63) module"]
pub type HUFFENC_AC1_63 = crate::Reg<u32, _HUFFENC_AC1_63>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_63;
#[doc = "`read()` method returns [huffenc_ac1_63::R](huffenc_ac1_63::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_63 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_63::W](huffenc_ac1_63::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_63 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_63;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_64](huffenc_ac1_64) module"]
pub type HUFFENC_AC1_64 = crate::Reg<u32, _HUFFENC_AC1_64>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_64;
#[doc = "`read()` method returns [huffenc_ac1_64::R](huffenc_ac1_64::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_64 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_64::W](huffenc_ac1_64::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_64 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_64;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_65](huffenc_ac1_65) module"]
pub type HUFFENC_AC1_65 = crate::Reg<u32, _HUFFENC_AC1_65>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_65;
#[doc = "`read()` method returns [huffenc_ac1_65::R](huffenc_ac1_65::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_65 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_65::W](huffenc_ac1_65::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_65 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_65;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_66](huffenc_ac1_66) module"]
pub type HUFFENC_AC1_66 = crate::Reg<u32, _HUFFENC_AC1_66>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_66;
#[doc = "`read()` method returns [huffenc_ac1_66::R](huffenc_ac1_66::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_66 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_66::W](huffenc_ac1_66::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_66 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_66;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_67](huffenc_ac1_67) module"]
pub type HUFFENC_AC1_67 = crate::Reg<u32, _HUFFENC_AC1_67>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_67;
#[doc = "`read()` method returns [huffenc_ac1_67::R](huffenc_ac1_67::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_67 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_67::W](huffenc_ac1_67::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_67 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_67;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_68](huffenc_ac1_68) module"]
pub type HUFFENC_AC1_68 = crate::Reg<u32, _HUFFENC_AC1_68>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_68;
#[doc = "`read()` method returns [huffenc_ac1_68::R](huffenc_ac1_68::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_68 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_68::W](huffenc_ac1_68::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_68 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_68;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_69](huffenc_ac1_69) module"]
pub type HUFFENC_AC1_69 = crate::Reg<u32, _HUFFENC_AC1_69>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_69;
#[doc = "`read()` method returns [huffenc_ac1_69::R](huffenc_ac1_69::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_69 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_69::W](huffenc_ac1_69::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_69 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_69;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_70](huffenc_ac1_70) module"]
pub type HUFFENC_AC1_70 = crate::Reg<u32, _HUFFENC_AC1_70>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_70;
#[doc = "`read()` method returns [huffenc_ac1_70::R](huffenc_ac1_70::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_70 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_70::W](huffenc_ac1_70::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_70 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_70;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_71](huffenc_ac1_71) module"]
pub type HUFFENC_AC1_71 = crate::Reg<u32, _HUFFENC_AC1_71>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_71;
#[doc = "`read()` method returns [huffenc_ac1_71::R](huffenc_ac1_71::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_71 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_71::W](huffenc_ac1_71::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_71 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_71;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_72](huffenc_ac1_72) module"]
pub type HUFFENC_AC1_72 = crate::Reg<u32, _HUFFENC_AC1_72>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_72;
#[doc = "`read()` method returns [huffenc_ac1_72::R](huffenc_ac1_72::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_72 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_72::W](huffenc_ac1_72::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_72 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_72;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_73](huffenc_ac1_73) module"]
pub type HUFFENC_AC1_73 = crate::Reg<u32, _HUFFENC_AC1_73>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_73;
#[doc = "`read()` method returns [huffenc_ac1_73::R](huffenc_ac1_73::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_73 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_73::W](huffenc_ac1_73::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_73 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_73;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_74](huffenc_ac1_74) module"]
pub type HUFFENC_AC1_74 = crate::Reg<u32, _HUFFENC_AC1_74>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_74;
#[doc = "`read()` method returns [huffenc_ac1_74::R](huffenc_ac1_74::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_74 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_74::W](huffenc_ac1_74::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_74 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_74;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_75](huffenc_ac1_75) module"]
pub type HUFFENC_AC1_75 = crate::Reg<u32, _HUFFENC_AC1_75>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_75;
#[doc = "`read()` method returns [huffenc_ac1_75::R](huffenc_ac1_75::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_75 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_75::W](huffenc_ac1_75::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_75 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_75;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_76](huffenc_ac1_76) module"]
pub type HUFFENC_AC1_76 = crate::Reg<u32, _HUFFENC_AC1_76>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_76;
#[doc = "`read()` method returns [huffenc_ac1_76::R](huffenc_ac1_76::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_76 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_76::W](huffenc_ac1_76::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_76 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_76;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_77](huffenc_ac1_77) module"]
pub type HUFFENC_AC1_77 = crate::Reg<u32, _HUFFENC_AC1_77>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_77;
#[doc = "`read()` method returns [huffenc_ac1_77::R](huffenc_ac1_77::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_77 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_77::W](huffenc_ac1_77::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_77 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_77;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_78](huffenc_ac1_78) module"]
pub type HUFFENC_AC1_78 = crate::Reg<u32, _HUFFENC_AC1_78>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_78;
#[doc = "`read()` method returns [huffenc_ac1_78::R](huffenc_ac1_78::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_78 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_78::W](huffenc_ac1_78::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_78 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_78;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_79](huffenc_ac1_79) module"]
pub type HUFFENC_AC1_79 = crate::Reg<u32, _HUFFENC_AC1_79>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_79;
#[doc = "`read()` method returns [huffenc_ac1_79::R](huffenc_ac1_79::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_79 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_79::W](huffenc_ac1_79::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_79 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_79;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_80](huffenc_ac1_80) module"]
pub type HUFFENC_AC1_80 = crate::Reg<u32, _HUFFENC_AC1_80>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_80;
#[doc = "`read()` method returns [huffenc_ac1_80::R](huffenc_ac1_80::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_80 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_80::W](huffenc_ac1_80::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_80 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_80;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_81](huffenc_ac1_81) module"]
pub type HUFFENC_AC1_81 = crate::Reg<u32, _HUFFENC_AC1_81>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_81;
#[doc = "`read()` method returns [huffenc_ac1_81::R](huffenc_ac1_81::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_81 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_81::W](huffenc_ac1_81::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_81 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_81;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_82](huffenc_ac1_82) module"]
pub type HUFFENC_AC1_82 = crate::Reg<u32, _HUFFENC_AC1_82>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_82;
#[doc = "`read()` method returns [huffenc_ac1_82::R](huffenc_ac1_82::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_82 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_82::W](huffenc_ac1_82::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_82 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_82;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_83](huffenc_ac1_83) module"]
pub type HUFFENC_AC1_83 = crate::Reg<u32, _HUFFENC_AC1_83>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_83;
#[doc = "`read()` method returns [huffenc_ac1_83::R](huffenc_ac1_83::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_83 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_83::W](huffenc_ac1_83::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_83 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_83;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_84](huffenc_ac1_84) module"]
pub type HUFFENC_AC1_84 = crate::Reg<u32, _HUFFENC_AC1_84>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_84;
#[doc = "`read()` method returns [huffenc_ac1_84::R](huffenc_ac1_84::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_84 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_84::W](huffenc_ac1_84::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_84 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_84;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_85](huffenc_ac1_85) module"]
pub type HUFFENC_AC1_85 = crate::Reg<u32, _HUFFENC_AC1_85>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_85;
#[doc = "`read()` method returns [huffenc_ac1_85::R](huffenc_ac1_85::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_85 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_85::W](huffenc_ac1_85::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_85 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_85;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_86](huffenc_ac1_86) module"]
pub type HUFFENC_AC1_86 = crate::Reg<u32, _HUFFENC_AC1_86>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_86;
#[doc = "`read()` method returns [huffenc_ac1_86::R](huffenc_ac1_86::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_86 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_86::W](huffenc_ac1_86::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_86 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_86;
#[doc = "JPEG encoder, AC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_ac1_87](huffenc_ac1_87) module"]
pub type HUFFENC_AC1_87 = crate::Reg<u32, _HUFFENC_AC1_87>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_AC1_87;
#[doc = "`read()` method returns [huffenc_ac1_87::R](huffenc_ac1_87::R) reader structure"]
impl crate::Readable for HUFFENC_AC1_87 {}
#[doc = "`write(|w| ..)` method takes [huffenc_ac1_87::W](huffenc_ac1_87::W) writer structure"]
impl crate::Writable for HUFFENC_AC1_87 {}
#[doc = "JPEG encoder, AC Huffman table 1"]
pub mod huffenc_ac1_87;
#[doc = "JPEG encoder, DC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_dc0_0](huffenc_dc0_0) module"]
pub type HUFFENC_DC0_0 = crate::Reg<u32, _HUFFENC_DC0_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_DC0_0;
#[doc = "`read()` method returns [huffenc_dc0_0::R](huffenc_dc0_0::R) reader structure"]
impl crate::Readable for HUFFENC_DC0_0 {}
#[doc = "`write(|w| ..)` method takes [huffenc_dc0_0::W](huffenc_dc0_0::W) writer structure"]
impl crate::Writable for HUFFENC_DC0_0 {}
#[doc = "JPEG encoder, DC Huffman table 0"]
pub mod huffenc_dc0_0;
#[doc = "JPEG encoder, DC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_dc0_1](huffenc_dc0_1) module"]
pub type HUFFENC_DC0_1 = crate::Reg<u32, _HUFFENC_DC0_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_DC0_1;
#[doc = "`read()` method returns [huffenc_dc0_1::R](huffenc_dc0_1::R) reader structure"]
impl crate::Readable for HUFFENC_DC0_1 {}
#[doc = "`write(|w| ..)` method takes [huffenc_dc0_1::W](huffenc_dc0_1::W) writer structure"]
impl crate::Writable for HUFFENC_DC0_1 {}
#[doc = "JPEG encoder, DC Huffman table 0"]
pub mod huffenc_dc0_1;
#[doc = "JPEG encoder, DC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_dc0_2](huffenc_dc0_2) module"]
pub type HUFFENC_DC0_2 = crate::Reg<u32, _HUFFENC_DC0_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_DC0_2;
#[doc = "`read()` method returns [huffenc_dc0_2::R](huffenc_dc0_2::R) reader structure"]
impl crate::Readable for HUFFENC_DC0_2 {}
#[doc = "`write(|w| ..)` method takes [huffenc_dc0_2::W](huffenc_dc0_2::W) writer structure"]
impl crate::Writable for HUFFENC_DC0_2 {}
#[doc = "JPEG encoder, DC Huffman table 0"]
pub mod huffenc_dc0_2;
#[doc = "JPEG encoder, DC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_dc0_3](huffenc_dc0_3) module"]
pub type HUFFENC_DC0_3 = crate::Reg<u32, _HUFFENC_DC0_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_DC0_3;
#[doc = "`read()` method returns [huffenc_dc0_3::R](huffenc_dc0_3::R) reader structure"]
impl crate::Readable for HUFFENC_DC0_3 {}
#[doc = "`write(|w| ..)` method takes [huffenc_dc0_3::W](huffenc_dc0_3::W) writer structure"]
impl crate::Writable for HUFFENC_DC0_3 {}
#[doc = "JPEG encoder, DC Huffman table 0"]
pub mod huffenc_dc0_3;
#[doc = "JPEG encoder, DC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_dc0_4](huffenc_dc0_4) module"]
pub type HUFFENC_DC0_4 = crate::Reg<u32, _HUFFENC_DC0_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_DC0_4;
#[doc = "`read()` method returns [huffenc_dc0_4::R](huffenc_dc0_4::R) reader structure"]
impl crate::Readable for HUFFENC_DC0_4 {}
#[doc = "`write(|w| ..)` method takes [huffenc_dc0_4::W](huffenc_dc0_4::W) writer structure"]
impl crate::Writable for HUFFENC_DC0_4 {}
#[doc = "JPEG encoder, DC Huffman table 0"]
pub mod huffenc_dc0_4;
#[doc = "JPEG encoder, DC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_dc0_5](huffenc_dc0_5) module"]
pub type HUFFENC_DC0_5 = crate::Reg<u32, _HUFFENC_DC0_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_DC0_5;
#[doc = "`read()` method returns [huffenc_dc0_5::R](huffenc_dc0_5::R) reader structure"]
impl crate::Readable for HUFFENC_DC0_5 {}
#[doc = "`write(|w| ..)` method takes [huffenc_dc0_5::W](huffenc_dc0_5::W) writer structure"]
impl crate::Writable for HUFFENC_DC0_5 {}
#[doc = "JPEG encoder, DC Huffman table 0"]
pub mod huffenc_dc0_5;
#[doc = "JPEG encoder, DC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_dc0_6](huffenc_dc0_6) module"]
pub type HUFFENC_DC0_6 = crate::Reg<u32, _HUFFENC_DC0_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_DC0_6;
#[doc = "`read()` method returns [huffenc_dc0_6::R](huffenc_dc0_6::R) reader structure"]
impl crate::Readable for HUFFENC_DC0_6 {}
#[doc = "`write(|w| ..)` method takes [huffenc_dc0_6::W](huffenc_dc0_6::W) writer structure"]
impl crate::Writable for HUFFENC_DC0_6 {}
#[doc = "JPEG encoder, DC Huffman table 0"]
pub mod huffenc_dc0_6;
#[doc = "JPEG encoder, DC Huffman table 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_dc0_7](huffenc_dc0_7) module"]
pub type HUFFENC_DC0_7 = crate::Reg<u32, _HUFFENC_DC0_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_DC0_7;
#[doc = "`read()` method returns [huffenc_dc0_7::R](huffenc_dc0_7::R) reader structure"]
impl crate::Readable for HUFFENC_DC0_7 {}
#[doc = "`write(|w| ..)` method takes [huffenc_dc0_7::W](huffenc_dc0_7::W) writer structure"]
impl crate::Writable for HUFFENC_DC0_7 {}
#[doc = "JPEG encoder, DC Huffman table 0"]
pub mod huffenc_dc0_7;
#[doc = "JPEG encoder, DC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_dc1_0](huffenc_dc1_0) module"]
pub type HUFFENC_DC1_0 = crate::Reg<u32, _HUFFENC_DC1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_DC1_0;
#[doc = "`read()` method returns [huffenc_dc1_0::R](huffenc_dc1_0::R) reader structure"]
impl crate::Readable for HUFFENC_DC1_0 {}
#[doc = "`write(|w| ..)` method takes [huffenc_dc1_0::W](huffenc_dc1_0::W) writer structure"]
impl crate::Writable for HUFFENC_DC1_0 {}
#[doc = "JPEG encoder, DC Huffman table 1"]
pub mod huffenc_dc1_0;
#[doc = "JPEG encoder, DC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_dc1_1](huffenc_dc1_1) module"]
pub type HUFFENC_DC1_1 = crate::Reg<u32, _HUFFENC_DC1_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_DC1_1;
#[doc = "`read()` method returns [huffenc_dc1_1::R](huffenc_dc1_1::R) reader structure"]
impl crate::Readable for HUFFENC_DC1_1 {}
#[doc = "`write(|w| ..)` method takes [huffenc_dc1_1::W](huffenc_dc1_1::W) writer structure"]
impl crate::Writable for HUFFENC_DC1_1 {}
#[doc = "JPEG encoder, DC Huffman table 1"]
pub mod huffenc_dc1_1;
#[doc = "JPEG encoder, DC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_dc1_2](huffenc_dc1_2) module"]
pub type HUFFENC_DC1_2 = crate::Reg<u32, _HUFFENC_DC1_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_DC1_2;
#[doc = "`read()` method returns [huffenc_dc1_2::R](huffenc_dc1_2::R) reader structure"]
impl crate::Readable for HUFFENC_DC1_2 {}
#[doc = "`write(|w| ..)` method takes [huffenc_dc1_2::W](huffenc_dc1_2::W) writer structure"]
impl crate::Writable for HUFFENC_DC1_2 {}
#[doc = "JPEG encoder, DC Huffman table 1"]
pub mod huffenc_dc1_2;
#[doc = "JPEG encoder, DC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_dc1_3](huffenc_dc1_3) module"]
pub type HUFFENC_DC1_3 = crate::Reg<u32, _HUFFENC_DC1_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_DC1_3;
#[doc = "`read()` method returns [huffenc_dc1_3::R](huffenc_dc1_3::R) reader structure"]
impl crate::Readable for HUFFENC_DC1_3 {}
#[doc = "`write(|w| ..)` method takes [huffenc_dc1_3::W](huffenc_dc1_3::W) writer structure"]
impl crate::Writable for HUFFENC_DC1_3 {}
#[doc = "JPEG encoder, DC Huffman table 1"]
pub mod huffenc_dc1_3;
#[doc = "JPEG encoder, DC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_dc1_4](huffenc_dc1_4) module"]
pub type HUFFENC_DC1_4 = crate::Reg<u32, _HUFFENC_DC1_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_DC1_4;
#[doc = "`read()` method returns [huffenc_dc1_4::R](huffenc_dc1_4::R) reader structure"]
impl crate::Readable for HUFFENC_DC1_4 {}
#[doc = "`write(|w| ..)` method takes [huffenc_dc1_4::W](huffenc_dc1_4::W) writer structure"]
impl crate::Writable for HUFFENC_DC1_4 {}
#[doc = "JPEG encoder, DC Huffman table 1"]
pub mod huffenc_dc1_4;
#[doc = "JPEG encoder, DC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_dc1_5](huffenc_dc1_5) module"]
pub type HUFFENC_DC1_5 = crate::Reg<u32, _HUFFENC_DC1_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_DC1_5;
#[doc = "`read()` method returns [huffenc_dc1_5::R](huffenc_dc1_5::R) reader structure"]
impl crate::Readable for HUFFENC_DC1_5 {}
#[doc = "`write(|w| ..)` method takes [huffenc_dc1_5::W](huffenc_dc1_5::W) writer structure"]
impl crate::Writable for HUFFENC_DC1_5 {}
#[doc = "JPEG encoder, DC Huffman table 1"]
pub mod huffenc_dc1_5;
#[doc = "JPEG encoder, DC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_dc1_6](huffenc_dc1_6) module"]
pub type HUFFENC_DC1_6 = crate::Reg<u32, _HUFFENC_DC1_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_DC1_6;
#[doc = "`read()` method returns [huffenc_dc1_6::R](huffenc_dc1_6::R) reader structure"]
impl crate::Readable for HUFFENC_DC1_6 {}
#[doc = "`write(|w| ..)` method takes [huffenc_dc1_6::W](huffenc_dc1_6::W) writer structure"]
impl crate::Writable for HUFFENC_DC1_6 {}
#[doc = "JPEG encoder, DC Huffman table 1"]
pub mod huffenc_dc1_6;
#[doc = "JPEG encoder, DC Huffman table 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffenc_dc1_7](huffenc_dc1_7) module"]
pub type HUFFENC_DC1_7 = crate::Reg<u32, _HUFFENC_DC1_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HUFFENC_DC1_7;
#[doc = "`read()` method returns [huffenc_dc1_7::R](huffenc_dc1_7::R) reader structure"]
impl crate::Readable for HUFFENC_DC1_7 {}
#[doc = "`write(|w| ..)` method takes [huffenc_dc1_7::W](huffenc_dc1_7::W) writer structure"]
impl crate::Writable for HUFFENC_DC1_7 {}
#[doc = "JPEG encoder, DC Huffman table 1"]
pub mod huffenc_dc1_7;
