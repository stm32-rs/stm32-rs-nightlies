#[doc = "Register `FDCAN_NDAT1` reader"]
pub type R = crate::R<FDCAN_NDAT1rs>;
#[doc = "Field `ND0` reader - New data"]
pub type ND0_R = crate::BitReader;
#[doc = "Field `ND1` reader - New data"]
pub type ND1_R = crate::BitReader;
#[doc = "Field `ND2` reader - New data"]
pub type ND2_R = crate::BitReader;
#[doc = "Field `ND3` reader - New data"]
pub type ND3_R = crate::BitReader;
#[doc = "Field `ND4` reader - New data"]
pub type ND4_R = crate::BitReader;
#[doc = "Field `ND5` reader - New data"]
pub type ND5_R = crate::BitReader;
#[doc = "Field `ND6` reader - New data"]
pub type ND6_R = crate::BitReader;
#[doc = "Field `ND7` reader - New data"]
pub type ND7_R = crate::BitReader;
#[doc = "Field `ND8` reader - New data"]
pub type ND8_R = crate::BitReader;
#[doc = "Field `ND9` reader - New data"]
pub type ND9_R = crate::BitReader;
#[doc = "Field `ND10` reader - New data"]
pub type ND10_R = crate::BitReader;
#[doc = "Field `ND11` reader - New data"]
pub type ND11_R = crate::BitReader;
#[doc = "Field `ND12` reader - New data"]
pub type ND12_R = crate::BitReader;
#[doc = "Field `ND13` reader - New data"]
pub type ND13_R = crate::BitReader;
#[doc = "Field `ND14` reader - New data"]
pub type ND14_R = crate::BitReader;
#[doc = "Field `ND15` reader - New data"]
pub type ND15_R = crate::BitReader;
#[doc = "Field `ND16` reader - New data"]
pub type ND16_R = crate::BitReader;
#[doc = "Field `ND17` reader - New data"]
pub type ND17_R = crate::BitReader;
#[doc = "Field `ND18` reader - New data"]
pub type ND18_R = crate::BitReader;
#[doc = "Field `ND19` reader - New data"]
pub type ND19_R = crate::BitReader;
#[doc = "Field `ND20` reader - New data"]
pub type ND20_R = crate::BitReader;
#[doc = "Field `ND21` reader - New data"]
pub type ND21_R = crate::BitReader;
#[doc = "Field `ND22` reader - New data"]
pub type ND22_R = crate::BitReader;
#[doc = "Field `ND23` reader - New data"]
pub type ND23_R = crate::BitReader;
#[doc = "Field `ND24` reader - New data"]
pub type ND24_R = crate::BitReader;
#[doc = "Field `ND25` reader - New data"]
pub type ND25_R = crate::BitReader;
#[doc = "Field `ND26` reader - New data"]
pub type ND26_R = crate::BitReader;
#[doc = "Field `ND27` reader - New data"]
pub type ND27_R = crate::BitReader;
#[doc = "Field `ND28` reader - New data"]
pub type ND28_R = crate::BitReader;
#[doc = "Field `ND29` reader - New data"]
pub type ND29_R = crate::BitReader;
#[doc = "Field `ND30` reader - New data"]
pub type ND30_R = crate::BitReader;
#[doc = "Field `ND31` reader - New data"]
pub type ND31_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - New data"]
    #[inline(always)]
    pub fn nd0(&self) -> ND0_R {
        ND0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - New data"]
    #[inline(always)]
    pub fn nd1(&self) -> ND1_R {
        ND1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - New data"]
    #[inline(always)]
    pub fn nd2(&self) -> ND2_R {
        ND2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - New data"]
    #[inline(always)]
    pub fn nd3(&self) -> ND3_R {
        ND3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - New data"]
    #[inline(always)]
    pub fn nd4(&self) -> ND4_R {
        ND4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - New data"]
    #[inline(always)]
    pub fn nd5(&self) -> ND5_R {
        ND5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - New data"]
    #[inline(always)]
    pub fn nd6(&self) -> ND6_R {
        ND6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - New data"]
    #[inline(always)]
    pub fn nd7(&self) -> ND7_R {
        ND7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - New data"]
    #[inline(always)]
    pub fn nd8(&self) -> ND8_R {
        ND8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - New data"]
    #[inline(always)]
    pub fn nd9(&self) -> ND9_R {
        ND9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - New data"]
    #[inline(always)]
    pub fn nd10(&self) -> ND10_R {
        ND10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - New data"]
    #[inline(always)]
    pub fn nd11(&self) -> ND11_R {
        ND11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - New data"]
    #[inline(always)]
    pub fn nd12(&self) -> ND12_R {
        ND12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - New data"]
    #[inline(always)]
    pub fn nd13(&self) -> ND13_R {
        ND13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - New data"]
    #[inline(always)]
    pub fn nd14(&self) -> ND14_R {
        ND14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - New data"]
    #[inline(always)]
    pub fn nd15(&self) -> ND15_R {
        ND15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - New data"]
    #[inline(always)]
    pub fn nd16(&self) -> ND16_R {
        ND16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - New data"]
    #[inline(always)]
    pub fn nd17(&self) -> ND17_R {
        ND17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - New data"]
    #[inline(always)]
    pub fn nd18(&self) -> ND18_R {
        ND18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - New data"]
    #[inline(always)]
    pub fn nd19(&self) -> ND19_R {
        ND19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - New data"]
    #[inline(always)]
    pub fn nd20(&self) -> ND20_R {
        ND20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - New data"]
    #[inline(always)]
    pub fn nd21(&self) -> ND21_R {
        ND21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - New data"]
    #[inline(always)]
    pub fn nd22(&self) -> ND22_R {
        ND22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - New data"]
    #[inline(always)]
    pub fn nd23(&self) -> ND23_R {
        ND23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - New data"]
    #[inline(always)]
    pub fn nd24(&self) -> ND24_R {
        ND24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - New data"]
    #[inline(always)]
    pub fn nd25(&self) -> ND25_R {
        ND25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - New data"]
    #[inline(always)]
    pub fn nd26(&self) -> ND26_R {
        ND26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - New data"]
    #[inline(always)]
    pub fn nd27(&self) -> ND27_R {
        ND27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - New data"]
    #[inline(always)]
    pub fn nd28(&self) -> ND28_R {
        ND28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - New data"]
    #[inline(always)]
    pub fn nd29(&self) -> ND29_R {
        ND29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - New data"]
    #[inline(always)]
    pub fn nd30(&self) -> ND30_R {
        ND30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - New data"]
    #[inline(always)]
    pub fn nd31(&self) -> ND31_R {
        ND31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "FDCAN New Data 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ndat1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_NDAT1rs;
impl crate::RegisterSpec for FDCAN_NDAT1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ndat1::R`](R) reader structure"]
impl crate::Readable for FDCAN_NDAT1rs {}
#[doc = "`reset()` method sets FDCAN_NDAT1 to value 0"]
impl crate::Resettable for FDCAN_NDAT1rs {
    const RESET_VALUE: u32 = 0;
}
