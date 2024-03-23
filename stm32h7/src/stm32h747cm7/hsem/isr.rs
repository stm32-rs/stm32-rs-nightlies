#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISRrs>;
#[doc = "Field `ISEM0` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM0_R = crate::BitReader;
#[doc = "Field `ISEM1` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM1_R = crate::BitReader;
#[doc = "Field `ISEM2` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM2_R = crate::BitReader;
#[doc = "Field `ISEM3` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM3_R = crate::BitReader;
#[doc = "Field `ISEM4` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM4_R = crate::BitReader;
#[doc = "Field `ISEM5` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM5_R = crate::BitReader;
#[doc = "Field `ISEM6` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM6_R = crate::BitReader;
#[doc = "Field `ISEM7` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM7_R = crate::BitReader;
#[doc = "Field `ISEM8` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM8_R = crate::BitReader;
#[doc = "Field `ISEM9` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM9_R = crate::BitReader;
#[doc = "Field `ISEM10` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM10_R = crate::BitReader;
#[doc = "Field `ISEM11` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM11_R = crate::BitReader;
#[doc = "Field `ISEM12` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM12_R = crate::BitReader;
#[doc = "Field `ISEM13` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM13_R = crate::BitReader;
#[doc = "Field `ISEM14` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM14_R = crate::BitReader;
#[doc = "Field `ISEM15` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM15_R = crate::BitReader;
#[doc = "Field `ISEM16` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM16_R = crate::BitReader;
#[doc = "Field `ISEM17` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM17_R = crate::BitReader;
#[doc = "Field `ISEM18` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM18_R = crate::BitReader;
#[doc = "Field `ISEM19` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM19_R = crate::BitReader;
#[doc = "Field `ISEM20` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM20_R = crate::BitReader;
#[doc = "Field `ISEM21` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM21_R = crate::BitReader;
#[doc = "Field `ISEM22` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM22_R = crate::BitReader;
#[doc = "Field `ISEM23` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM23_R = crate::BitReader;
#[doc = "Field `ISEM24` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM24_R = crate::BitReader;
#[doc = "Field `ISEM25` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM25_R = crate::BitReader;
#[doc = "Field `ISEM26` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM26_R = crate::BitReader;
#[doc = "Field `ISEM27` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM27_R = crate::BitReader;
#[doc = "Field `ISEM28` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM28_R = crate::BitReader;
#[doc = "Field `ISEM29` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM29_R = crate::BitReader;
#[doc = "Field `ISEM30` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM30_R = crate::BitReader;
#[doc = "Field `ISEM31` reader - Interrupt(N) semaphore n status bit before enable (mask)"]
pub type ISEM31_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem0(&self) -> ISEM0_R {
        ISEM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem1(&self) -> ISEM1_R {
        ISEM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem2(&self) -> ISEM2_R {
        ISEM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem3(&self) -> ISEM3_R {
        ISEM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem4(&self) -> ISEM4_R {
        ISEM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem5(&self) -> ISEM5_R {
        ISEM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem6(&self) -> ISEM6_R {
        ISEM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem7(&self) -> ISEM7_R {
        ISEM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem8(&self) -> ISEM8_R {
        ISEM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem9(&self) -> ISEM9_R {
        ISEM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem10(&self) -> ISEM10_R {
        ISEM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem11(&self) -> ISEM11_R {
        ISEM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem12(&self) -> ISEM12_R {
        ISEM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem13(&self) -> ISEM13_R {
        ISEM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem14(&self) -> ISEM14_R {
        ISEM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem15(&self) -> ISEM15_R {
        ISEM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem16(&self) -> ISEM16_R {
        ISEM16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem17(&self) -> ISEM17_R {
        ISEM17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem18(&self) -> ISEM18_R {
        ISEM18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem19(&self) -> ISEM19_R {
        ISEM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem20(&self) -> ISEM20_R {
        ISEM20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem21(&self) -> ISEM21_R {
        ISEM21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem22(&self) -> ISEM22_R {
        ISEM22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem23(&self) -> ISEM23_R {
        ISEM23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem24(&self) -> ISEM24_R {
        ISEM24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem25(&self) -> ISEM25_R {
        ISEM25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem26(&self) -> ISEM26_R {
        ISEM26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem27(&self) -> ISEM27_R {
        ISEM27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem28(&self) -> ISEM28_R {
        ISEM28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem29(&self) -> ISEM29_R {
        ISEM29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem30(&self) -> ISEM30_R {
        ISEM30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt(N) semaphore n status bit before enable (mask)"]
    #[inline(always)]
    pub fn isem31(&self) -> ISEM31_R {
        ISEM31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "HSEM Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISRrs {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0;
}
