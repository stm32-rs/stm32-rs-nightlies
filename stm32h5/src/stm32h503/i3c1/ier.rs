///Register `IER` reader
pub type R = crate::R<IERrs>;
///Field `CFNFIE` reader - C-FIFO not full interrupt enable (whatever the I3C is acting as controller/target)
pub type CFNFIE_R = crate::BitReader;
///Field `SFNEIE` reader - S-FIFO not empty interrupt enable (whatever the I3C is acting as controller/target)
pub type SFNEIE_R = crate::BitReader;
///Field `TXFNFIE` reader - TX-FIFO not full interrupt enable (whatever the I3C is acting as controller/target)
pub type TXFNFIE_R = crate::BitReader;
///Field `RXFNEIE` reader - RX-FIFO not empty interrupt enable (whatever the I3C is acting as controller/target)
pub type RXFNEIE_R = crate::BitReader;
///Field `FCIE` reader - frame complete interrupt enable (whatever the I3C is acting as controller/target)
pub type FCIE_R = crate::BitReader;
///Field `RXTGTENDIE` reader - target-initiated read end interrupt enable (when the I3C is acting as controller)
pub type RXTGTENDIE_R = crate::BitReader;
///Field `ERRIE` reader - error interrupt enable (whatever the I3C is acting as controller/target)
pub type ERRIE_R = crate::BitReader;
///Field `IBIIE` reader - IBI request interrupt enable (when the I3C is acting as controller)
pub type IBIIE_R = crate::BitReader;
///Field `IBIENDIE` reader - IBI end interrupt enable (when the I3C is acting as target)
pub type IBIENDIE_R = crate::BitReader;
///Field `CRIE` reader - controller-role request interrupt enable (when the I3C is acting as controller)
pub type CRIE_R = crate::BitReader;
///Field `CRUPDIE` reader - controller-role update interrupt enable (when the I3C is acting as target)
pub type CRUPDIE_R = crate::BitReader;
///Field `HJIE` reader - hot-join interrupt enable (when the I3C is acting as controller)
pub type HJIE_R = crate::BitReader;
///Field `WKPIE` reader - wakeup interrupt enable (when the I3C is acting as target)
pub type WKPIE_R = crate::BitReader;
///Field `GETIE` reader - GETxxx CCC interrupt enable (when the I3C is acting as target)
pub type GETIE_R = crate::BitReader;
///Field `STAIE` reader - GETSTATUS CCC interrupt enable (when the I3C is acting as target)
pub type STAIE_R = crate::BitReader;
///Field `DAUPDIE` reader - ENTDAA/RSTDAA/SETNEWDA CCC interrupt enable (when the I3C is acting as target)
pub type DAUPDIE_R = crate::BitReader;
///Field `MWLUPDIE` reader - SETMWL CCC interrupt enable (when the I3C is acting as target)
pub type MWLUPDIE_R = crate::BitReader;
///Field `MRLUPDIE` reader - SETMRL CCC interrupt enable (when the I3C is acting as target)
pub type MRLUPDIE_R = crate::BitReader;
///Field `RSTIE` reader - reset pattern interrupt enable (when the I3C is acting as target)
pub type RSTIE_R = crate::BitReader;
///Field `ASUPDIE` reader - ENTASx CCC interrupt enable (when the I3C is acting as target)
pub type ASUPDIE_R = crate::BitReader;
///Field `INTUPDIE` reader - ENEC/DISEC CCC interrupt enable (when the I3C is acting as target)
pub type INTUPDIE_R = crate::BitReader;
///Field `DEFIE` reader - DEFTGTS CCC interrupt enable (when the I3C is acting as target)
pub type DEFIE_R = crate::BitReader;
///Field `GRPIE` reader - DEFGRPA CCC interrupt enable (when the I3C is acting as target)
pub type GRPIE_R = crate::BitReader;
impl R {
    ///Bit 2 - C-FIFO not full interrupt enable (whatever the I3C is acting as controller/target)
    #[inline(always)]
    pub fn cfnfie(&self) -> CFNFIE_R {
        CFNFIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - S-FIFO not empty interrupt enable (whatever the I3C is acting as controller/target)
    #[inline(always)]
    pub fn sfneie(&self) -> SFNEIE_R {
        SFNEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TX-FIFO not full interrupt enable (whatever the I3C is acting as controller/target)
    #[inline(always)]
    pub fn txfnfie(&self) -> TXFNFIE_R {
        TXFNFIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RX-FIFO not empty interrupt enable (whatever the I3C is acting as controller/target)
    #[inline(always)]
    pub fn rxfneie(&self) -> RXFNEIE_R {
        RXFNEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 9 - frame complete interrupt enable (whatever the I3C is acting as controller/target)
    #[inline(always)]
    pub fn fcie(&self) -> FCIE_R {
        FCIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - target-initiated read end interrupt enable (when the I3C is acting as controller)
    #[inline(always)]
    pub fn rxtgtendie(&self) -> RXTGTENDIE_R {
        RXTGTENDIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - error interrupt enable (whatever the I3C is acting as controller/target)
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - IBI request interrupt enable (when the I3C is acting as controller)
    #[inline(always)]
    pub fn ibiie(&self) -> IBIIE_R {
        IBIIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - IBI end interrupt enable (when the I3C is acting as target)
    #[inline(always)]
    pub fn ibiendie(&self) -> IBIENDIE_R {
        IBIENDIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - controller-role request interrupt enable (when the I3C is acting as controller)
    #[inline(always)]
    pub fn crie(&self) -> CRIE_R {
        CRIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - controller-role update interrupt enable (when the I3C is acting as target)
    #[inline(always)]
    pub fn crupdie(&self) -> CRUPDIE_R {
        CRUPDIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - hot-join interrupt enable (when the I3C is acting as controller)
    #[inline(always)]
    pub fn hjie(&self) -> HJIE_R {
        HJIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - wakeup interrupt enable (when the I3C is acting as target)
    #[inline(always)]
    pub fn wkpie(&self) -> WKPIE_R {
        WKPIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - GETxxx CCC interrupt enable (when the I3C is acting as target)
    #[inline(always)]
    pub fn getie(&self) -> GETIE_R {
        GETIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - GETSTATUS CCC interrupt enable (when the I3C is acting as target)
    #[inline(always)]
    pub fn staie(&self) -> STAIE_R {
        STAIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - ENTDAA/RSTDAA/SETNEWDA CCC interrupt enable (when the I3C is acting as target)
    #[inline(always)]
    pub fn daupdie(&self) -> DAUPDIE_R {
        DAUPDIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - SETMWL CCC interrupt enable (when the I3C is acting as target)
    #[inline(always)]
    pub fn mwlupdie(&self) -> MWLUPDIE_R {
        MWLUPDIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - SETMRL CCC interrupt enable (when the I3C is acting as target)
    #[inline(always)]
    pub fn mrlupdie(&self) -> MRLUPDIE_R {
        MRLUPDIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - reset pattern interrupt enable (when the I3C is acting as target)
    #[inline(always)]
    pub fn rstie(&self) -> RSTIE_R {
        RSTIE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - ENTASx CCC interrupt enable (when the I3C is acting as target)
    #[inline(always)]
    pub fn asupdie(&self) -> ASUPDIE_R {
        ASUPDIE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - ENEC/DISEC CCC interrupt enable (when the I3C is acting as target)
    #[inline(always)]
    pub fn intupdie(&self) -> INTUPDIE_R {
        INTUPDIE_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - DEFTGTS CCC interrupt enable (when the I3C is acting as target)
    #[inline(always)]
    pub fn defie(&self) -> DEFIE_R {
        DEFIE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - DEFGRPA CCC interrupt enable (when the I3C is acting as target)
    #[inline(always)]
    pub fn grpie(&self) -> GRPIE_R {
        GRPIE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("cfnfie", &self.cfnfie())
            .field("sfneie", &self.sfneie())
            .field("txfnfie", &self.txfnfie())
            .field("rxfneie", &self.rxfneie())
            .field("fcie", &self.fcie())
            .field("rxtgtendie", &self.rxtgtendie())
            .field("errie", &self.errie())
            .field("ibiie", &self.ibiie())
            .field("ibiendie", &self.ibiendie())
            .field("crie", &self.crie())
            .field("crupdie", &self.crupdie())
            .field("hjie", &self.hjie())
            .field("wkpie", &self.wkpie())
            .field("getie", &self.getie())
            .field("staie", &self.staie())
            .field("daupdie", &self.daupdie())
            .field("mwlupdie", &self.mwlupdie())
            .field("mrlupdie", &self.mrlupdie())
            .field("rstie", &self.rstie())
            .field("asupdie", &self.asupdie())
            .field("intupdie", &self.intupdie())
            .field("defie", &self.defie())
            .field("grpie", &self.grpie())
            .finish()
    }
}
/**I3C interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:IER)*/
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {}
