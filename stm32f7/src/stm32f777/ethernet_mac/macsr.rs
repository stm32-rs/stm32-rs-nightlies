///Register `MACSR` reader
pub type R = crate::R<MACSRrs>;
///Register `MACSR` writer
pub type W = crate::W<MACSRrs>;
///Field `PMTS` reader - PMT status
pub type PMTS_R = crate::BitReader;
///Field `MMCS` reader - MMC status
pub type MMCS_R = crate::BitReader;
///Field `MMCRS` reader - MMC receive status
pub type MMCRS_R = crate::BitReader;
///Field `MMCTS` reader - MMC transmit status
pub type MMCTS_R = crate::BitReader;
///Field `TSTS` reader - Time stamp trigger status
pub type TSTS_R = crate::BitReader;
///Field `TSTS` writer - Time stamp trigger status
pub type TSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 3 - PMT status
    #[inline(always)]
    pub fn pmts(&self) -> PMTS_R {
        PMTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - MMC status
    #[inline(always)]
    pub fn mmcs(&self) -> MMCS_R {
        MMCS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MMC receive status
    #[inline(always)]
    pub fn mmcrs(&self) -> MMCRS_R {
        MMCRS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - MMC transmit status
    #[inline(always)]
    pub fn mmcts(&self) -> MMCTS_R {
        MMCTS_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - Time stamp trigger status
    #[inline(always)]
    pub fn tsts(&self) -> TSTS_R {
        TSTS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACSR")
            .field("pmts", &self.pmts())
            .field("mmcs", &self.mmcs())
            .field("mmcrs", &self.mmcrs())
            .field("mmcts", &self.mmcts())
            .field("tsts", &self.tsts())
            .finish()
    }
}
impl W {
    ///Bit 9 - Time stamp trigger status
    #[inline(always)]
    pub fn tsts(&mut self) -> TSTS_W<'_, MACSRrs> {
        TSTS_W::new(self, 9)
    }
}
/**Ethernet MAC interrupt status register

You can [`read`](crate::Reg::read) this register and get [`macsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F777.html#Ethernet_MAC:MACSR)*/
pub struct MACSRrs;
impl crate::RegisterSpec for MACSRrs {
    type Ux = u32;
}
///`read()` method returns [`macsr::R`](R) reader structure
impl crate::Readable for MACSRrs {}
///`write(|w| ..)` method takes [`macsr::W`](W) writer structure
impl crate::Writable for MACSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACSR to value 0
impl crate::Resettable for MACSRrs {}
