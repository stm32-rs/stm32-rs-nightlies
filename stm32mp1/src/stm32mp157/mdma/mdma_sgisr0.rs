#[doc = "Register `MDMA_SGISR0` reader"]
pub type R = crate::R<MDMA_SGISR0rs>;
#[doc = "Field `GIF0` reader - GIF0"]
pub type GIF0_R = crate::BitReader;
#[doc = "Field `GIF1` reader - GIF1"]
pub type GIF1_R = crate::BitReader;
#[doc = "Field `GIF2` reader - GIF2"]
pub type GIF2_R = crate::BitReader;
#[doc = "Field `GIF3` reader - GIF3"]
pub type GIF3_R = crate::BitReader;
#[doc = "Field `GIF4` reader - GIF4"]
pub type GIF4_R = crate::BitReader;
#[doc = "Field `GIF5` reader - GIF5"]
pub type GIF5_R = crate::BitReader;
#[doc = "Field `GIF6` reader - GIF6"]
pub type GIF6_R = crate::BitReader;
#[doc = "Field `GIF7` reader - GIF7"]
pub type GIF7_R = crate::BitReader;
#[doc = "Field `GIF8` reader - GIF8"]
pub type GIF8_R = crate::BitReader;
#[doc = "Field `GIF9` reader - GIF9"]
pub type GIF9_R = crate::BitReader;
#[doc = "Field `GIF10` reader - GIF10"]
pub type GIF10_R = crate::BitReader;
#[doc = "Field `GIF11` reader - GIF11"]
pub type GIF11_R = crate::BitReader;
#[doc = "Field `GIF12` reader - GIF12"]
pub type GIF12_R = crate::BitReader;
#[doc = "Field `GIF13` reader - GIF13"]
pub type GIF13_R = crate::BitReader;
#[doc = "Field `GIF14` reader - GIF14"]
pub type GIF14_R = crate::BitReader;
#[doc = "Field `GIF15` reader - GIF15"]
pub type GIF15_R = crate::BitReader;
#[doc = "Field `GIF16` reader - GIF16"]
pub type GIF16_R = crate::BitReader;
#[doc = "Field `GIF17` reader - GIF17"]
pub type GIF17_R = crate::BitReader;
#[doc = "Field `GIF18` reader - GIF18"]
pub type GIF18_R = crate::BitReader;
#[doc = "Field `GIF19` reader - GIF19"]
pub type GIF19_R = crate::BitReader;
#[doc = "Field `GIF20` reader - GIF20"]
pub type GIF20_R = crate::BitReader;
#[doc = "Field `GIF21` reader - GIF21"]
pub type GIF21_R = crate::BitReader;
#[doc = "Field `GIF22` reader - GIF22"]
pub type GIF22_R = crate::BitReader;
#[doc = "Field `GIF23` reader - GIF23"]
pub type GIF23_R = crate::BitReader;
#[doc = "Field `GIF24` reader - GIF24"]
pub type GIF24_R = crate::BitReader;
#[doc = "Field `GIF25` reader - GIF25"]
pub type GIF25_R = crate::BitReader;
#[doc = "Field `GIF26` reader - GIF26"]
pub type GIF26_R = crate::BitReader;
#[doc = "Field `GIF27` reader - GIF27"]
pub type GIF27_R = crate::BitReader;
#[doc = "Field `GIF28` reader - GIF28"]
pub type GIF28_R = crate::BitReader;
#[doc = "Field `GIF29` reader - GIF29"]
pub type GIF29_R = crate::BitReader;
#[doc = "Field `GIF30` reader - GIF30"]
pub type GIF30_R = crate::BitReader;
#[doc = "Field `GIF31` reader - GIF31"]
pub type GIF31_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - GIF0"]
    #[inline(always)]
    pub fn gif0(&self) -> GIF0_R {
        GIF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GIF1"]
    #[inline(always)]
    pub fn gif1(&self) -> GIF1_R {
        GIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GIF2"]
    #[inline(always)]
    pub fn gif2(&self) -> GIF2_R {
        GIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GIF3"]
    #[inline(always)]
    pub fn gif3(&self) -> GIF3_R {
        GIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GIF4"]
    #[inline(always)]
    pub fn gif4(&self) -> GIF4_R {
        GIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GIF5"]
    #[inline(always)]
    pub fn gif5(&self) -> GIF5_R {
        GIF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GIF6"]
    #[inline(always)]
    pub fn gif6(&self) -> GIF6_R {
        GIF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GIF7"]
    #[inline(always)]
    pub fn gif7(&self) -> GIF7_R {
        GIF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GIF8"]
    #[inline(always)]
    pub fn gif8(&self) -> GIF8_R {
        GIF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GIF9"]
    #[inline(always)]
    pub fn gif9(&self) -> GIF9_R {
        GIF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GIF10"]
    #[inline(always)]
    pub fn gif10(&self) -> GIF10_R {
        GIF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GIF11"]
    #[inline(always)]
    pub fn gif11(&self) -> GIF11_R {
        GIF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GIF12"]
    #[inline(always)]
    pub fn gif12(&self) -> GIF12_R {
        GIF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GIF13"]
    #[inline(always)]
    pub fn gif13(&self) -> GIF13_R {
        GIF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GIF14"]
    #[inline(always)]
    pub fn gif14(&self) -> GIF14_R {
        GIF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GIF15"]
    #[inline(always)]
    pub fn gif15(&self) -> GIF15_R {
        GIF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - GIF16"]
    #[inline(always)]
    pub fn gif16(&self) -> GIF16_R {
        GIF16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GIF17"]
    #[inline(always)]
    pub fn gif17(&self) -> GIF17_R {
        GIF17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GIF18"]
    #[inline(always)]
    pub fn gif18(&self) -> GIF18_R {
        GIF18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GIF19"]
    #[inline(always)]
    pub fn gif19(&self) -> GIF19_R {
        GIF19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GIF20"]
    #[inline(always)]
    pub fn gif20(&self) -> GIF20_R {
        GIF20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - GIF21"]
    #[inline(always)]
    pub fn gif21(&self) -> GIF21_R {
        GIF21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - GIF22"]
    #[inline(always)]
    pub fn gif22(&self) -> GIF22_R {
        GIF22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GIF23"]
    #[inline(always)]
    pub fn gif23(&self) -> GIF23_R {
        GIF23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GIF24"]
    #[inline(always)]
    pub fn gif24(&self) -> GIF24_R {
        GIF24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GIF25"]
    #[inline(always)]
    pub fn gif25(&self) -> GIF25_R {
        GIF25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - GIF26"]
    #[inline(always)]
    pub fn gif26(&self) -> GIF26_R {
        GIF26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - GIF27"]
    #[inline(always)]
    pub fn gif27(&self) -> GIF27_R {
        GIF27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - GIF28"]
    #[inline(always)]
    pub fn gif28(&self) -> GIF28_R {
        GIF28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - GIF29"]
    #[inline(always)]
    pub fn gif29(&self) -> GIF29_R {
        GIF29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - GIF30"]
    #[inline(always)]
    pub fn gif30(&self) -> GIF30_R {
        GIF30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GIF31"]
    #[inline(always)]
    pub fn gif31(&self) -> GIF31_R {
        GIF31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "MDMA secure global interrupt/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_sgisr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_SGISR0rs;
impl crate::RegisterSpec for MDMA_SGISR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_sgisr0::R`](R) reader structure"]
impl crate::Readable for MDMA_SGISR0rs {}
#[doc = "`reset()` method sets MDMA_SGISR0 to value 0"]
impl crate::Resettable for MDMA_SGISR0rs {
    const RESET_VALUE: u32 = 0;
}
