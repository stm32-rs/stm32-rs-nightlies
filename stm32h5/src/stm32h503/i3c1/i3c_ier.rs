#[doc = "Register `I3C_IER` reader"]
pub type R = crate::R<I3C_IERrs>;
#[doc = "Field `CFNFIE` reader - C-FIFO not full interrupt enable (whatever the I3C is acting as controller/target)"]
pub type CFNFIE_R = crate::BitReader;
#[doc = "Field `SFNEIE` reader - S-FIFO not empty interrupt enable (whatever the I3C is acting as controller/target)"]
pub type SFNEIE_R = crate::BitReader;
#[doc = "Field `TXFNFIE` reader - TX-FIFO not full interrupt enable (whatever the I3C is acting as controller/target)"]
pub type TXFNFIE_R = crate::BitReader;
#[doc = "Field `RXFNEIE` reader - RX-FIFO not empty interrupt enable (whatever the I3C is acting as controller/target)"]
pub type RXFNEIE_R = crate::BitReader;
#[doc = "Field `FCIE` reader - frame complete interrupt enable (whatever the I3C is acting as controller/target)"]
pub type FCIE_R = crate::BitReader;
#[doc = "Field `RXTGTENDIE` reader - target-initiated read end interrupt enable (when the I3C is acting as controller)"]
pub type RXTGTENDIE_R = crate::BitReader;
#[doc = "Field `ERRIE` reader - error interrupt enable (whatever the I3C is acting as controller/target)"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `IBIIE` reader - IBI request interrupt enable (when the I3C is acting as controller)"]
pub type IBIIE_R = crate::BitReader;
#[doc = "Field `IBIENDIE` reader - IBI end interrupt enable (when the I3C is acting as target)"]
pub type IBIENDIE_R = crate::BitReader;
#[doc = "Field `CRIE` reader - controller-role request interrupt enable (when the I3C is acting as controller)"]
pub type CRIE_R = crate::BitReader;
#[doc = "Field `CRUPDIE` reader - controller-role update interrupt enable (when the I3C is acting as target)"]
pub type CRUPDIE_R = crate::BitReader;
#[doc = "Field `HJIE` reader - hot-join interrupt enable (when the I3C is acting as controller)"]
pub type HJIE_R = crate::BitReader;
#[doc = "Field `WKPIE` reader - wakeup interrupt enable (when the I3C is acting as target)"]
pub type WKPIE_R = crate::BitReader;
#[doc = "Field `GETIE` reader - GETxxx CCC interrupt enable (when the I3C is acting as target)"]
pub type GETIE_R = crate::BitReader;
#[doc = "Field `STAIE` reader - GETSTATUS CCC interrupt enable (when the I3C is acting as target)"]
pub type STAIE_R = crate::BitReader;
#[doc = "Field `DAUPDIE` reader - ENTDAA/RSTDAA/SETNEWDA CCC interrupt enable (when the I3C is acting as target)"]
pub type DAUPDIE_R = crate::BitReader;
#[doc = "Field `MWLUPDIE` reader - SETMWL CCC interrupt enable (when the I3C is acting as target)"]
pub type MWLUPDIE_R = crate::BitReader;
#[doc = "Field `MRLUPDIE` reader - SETMRL CCC interrupt enable (when the I3C is acting as target)"]
pub type MRLUPDIE_R = crate::BitReader;
#[doc = "Field `RSTIE` reader - reset pattern interrupt enable (when the I3C is acting as target)"]
pub type RSTIE_R = crate::BitReader;
#[doc = "Field `ASUPDIE` reader - ENTASx CCC interrupt enable (when the I3C is acting as target)"]
pub type ASUPDIE_R = crate::BitReader;
#[doc = "Field `INTUPDIE` reader - ENEC/DISEC CCC interrupt enable (when the I3C is acting as target)"]
pub type INTUPDIE_R = crate::BitReader;
#[doc = "Field `DEFIE` reader - DEFTGTS CCC interrupt enable (when the I3C is acting as target)"]
pub type DEFIE_R = crate::BitReader;
#[doc = "Field `GRPIE` reader - DEFGRPA CCC interrupt enable (when the I3C is acting as target)"]
pub type GRPIE_R = crate::BitReader;
impl R {
    #[doc = "Bit 2 - C-FIFO not full interrupt enable (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub fn cfnfie(&self) -> CFNFIE_R {
        CFNFIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - S-FIFO not empty interrupt enable (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub fn sfneie(&self) -> SFNEIE_R {
        SFNEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX-FIFO not full interrupt enable (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub fn txfnfie(&self) -> TXFNFIE_R {
        TXFNFIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RX-FIFO not empty interrupt enable (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub fn rxfneie(&self) -> RXFNEIE_R {
        RXFNEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - frame complete interrupt enable (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub fn fcie(&self) -> FCIE_R {
        FCIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - target-initiated read end interrupt enable (when the I3C is acting as controller)"]
    #[inline(always)]
    pub fn rxtgtendie(&self) -> RXTGTENDIE_R {
        RXTGTENDIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - error interrupt enable (whatever the I3C is acting as controller/target)"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - IBI request interrupt enable (when the I3C is acting as controller)"]
    #[inline(always)]
    pub fn ibiie(&self) -> IBIIE_R {
        IBIIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - IBI end interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn ibiendie(&self) -> IBIENDIE_R {
        IBIENDIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - controller-role request interrupt enable (when the I3C is acting as controller)"]
    #[inline(always)]
    pub fn crie(&self) -> CRIE_R {
        CRIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - controller-role update interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn crupdie(&self) -> CRUPDIE_R {
        CRUPDIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - hot-join interrupt enable (when the I3C is acting as controller)"]
    #[inline(always)]
    pub fn hjie(&self) -> HJIE_R {
        HJIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - wakeup interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn wkpie(&self) -> WKPIE_R {
        WKPIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - GETxxx CCC interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn getie(&self) -> GETIE_R {
        GETIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GETSTATUS CCC interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn staie(&self) -> STAIE_R {
        STAIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ENTDAA/RSTDAA/SETNEWDA CCC interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn daupdie(&self) -> DAUPDIE_R {
        DAUPDIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SETMWL CCC interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn mwlupdie(&self) -> MWLUPDIE_R {
        MWLUPDIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SETMRL CCC interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn mrlupdie(&self) -> MRLUPDIE_R {
        MRLUPDIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - reset pattern interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn rstie(&self) -> RSTIE_R {
        RSTIE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - ENTASx CCC interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn asupdie(&self) -> ASUPDIE_R {
        ASUPDIE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ENEC/DISEC CCC interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn intupdie(&self) -> INTUPDIE_R {
        INTUPDIE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DEFTGTS CCC interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn defie(&self) -> DEFIE_R {
        DEFIE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DEFGRPA CCC interrupt enable (when the I3C is acting as target)"]
    #[inline(always)]
    pub fn grpie(&self) -> GRPIE_R {
        GRPIE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "I3C interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i3c_ier::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3C_IERrs;
impl crate::RegisterSpec for I3C_IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3c_ier::R`](R) reader structure"]
impl crate::Readable for I3C_IERrs {}
#[doc = "`reset()` method sets I3C_IER to value 0"]
impl crate::Resettable for I3C_IERrs {
    const RESET_VALUE: u32 = 0;
}
