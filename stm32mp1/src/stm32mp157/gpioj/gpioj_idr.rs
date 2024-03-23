#[doc = "Register `GPIOJ_IDR` reader"]
pub type R = crate::R<GPIOJ_IDRrs>;
#[doc = "Field `IDR0` reader - IDR0"]
pub type IDR0_R = crate::BitReader;
#[doc = "Field `IDR1` reader - IDR1"]
pub type IDR1_R = crate::BitReader;
#[doc = "Field `IDR2` reader - IDR2"]
pub type IDR2_R = crate::BitReader;
#[doc = "Field `IDR3` reader - IDR3"]
pub type IDR3_R = crate::BitReader;
#[doc = "Field `IDR4` reader - IDR4"]
pub type IDR4_R = crate::BitReader;
#[doc = "Field `IDR5` reader - IDR5"]
pub type IDR5_R = crate::BitReader;
#[doc = "Field `IDR6` reader - IDR6"]
pub type IDR6_R = crate::BitReader;
#[doc = "Field `IDR7` reader - IDR7"]
pub type IDR7_R = crate::BitReader;
#[doc = "Field `IDR8` reader - IDR8"]
pub type IDR8_R = crate::BitReader;
#[doc = "Field `IDR9` reader - IDR9"]
pub type IDR9_R = crate::BitReader;
#[doc = "Field `IDR10` reader - IDR10"]
pub type IDR10_R = crate::BitReader;
#[doc = "Field `IDR11` reader - IDR11"]
pub type IDR11_R = crate::BitReader;
#[doc = "Field `IDR12` reader - IDR12"]
pub type IDR12_R = crate::BitReader;
#[doc = "Field `IDR13` reader - IDR13"]
pub type IDR13_R = crate::BitReader;
#[doc = "Field `IDR14` reader - IDR14"]
pub type IDR14_R = crate::BitReader;
#[doc = "Field `IDR15` reader - IDR15"]
pub type IDR15_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - IDR0"]
    #[inline(always)]
    pub fn idr0(&self) -> IDR0_R {
        IDR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IDR1"]
    #[inline(always)]
    pub fn idr1(&self) -> IDR1_R {
        IDR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IDR2"]
    #[inline(always)]
    pub fn idr2(&self) -> IDR2_R {
        IDR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IDR3"]
    #[inline(always)]
    pub fn idr3(&self) -> IDR3_R {
        IDR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDR4"]
    #[inline(always)]
    pub fn idr4(&self) -> IDR4_R {
        IDR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IDR5"]
    #[inline(always)]
    pub fn idr5(&self) -> IDR5_R {
        IDR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IDR6"]
    #[inline(always)]
    pub fn idr6(&self) -> IDR6_R {
        IDR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IDR7"]
    #[inline(always)]
    pub fn idr7(&self) -> IDR7_R {
        IDR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IDR8"]
    #[inline(always)]
    pub fn idr8(&self) -> IDR8_R {
        IDR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IDR9"]
    #[inline(always)]
    pub fn idr9(&self) -> IDR9_R {
        IDR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IDR10"]
    #[inline(always)]
    pub fn idr10(&self) -> IDR10_R {
        IDR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - IDR11"]
    #[inline(always)]
    pub fn idr11(&self) -> IDR11_R {
        IDR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - IDR12"]
    #[inline(always)]
    pub fn idr12(&self) -> IDR12_R {
        IDR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - IDR13"]
    #[inline(always)]
    pub fn idr13(&self) -> IDR13_R {
        IDR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - IDR14"]
    #[inline(always)]
    pub fn idr14(&self) -> IDR14_R {
        IDR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - IDR15"]
    #[inline(always)]
    pub fn idr15(&self) -> IDR15_R {
        IDR15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "GPIO port input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioj_idr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOJ_IDRrs;
impl crate::RegisterSpec for GPIOJ_IDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioj_idr::R`](R) reader structure"]
impl crate::Readable for GPIOJ_IDRrs {}
#[doc = "`reset()` method sets GPIOJ_IDR to value 0"]
impl crate::Resettable for GPIOJ_IDRrs {
    const RESET_VALUE: u32 = 0;
}
