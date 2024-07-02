///Register `ETH_MACLMIR` reader
pub type R = crate::R<ETH_MACLMIRrs>;
///Register `ETH_MACLMIR` writer
pub type W = crate::W<ETH_MACLMIRrs>;
///Field `LSI` reader - LSI
pub type LSI_R = crate::FieldReader;
///Field `LSI` writer - LSI
pub type LSI_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DRSYNCR` reader - DRSYNCR
pub type DRSYNCR_R = crate::FieldReader;
///Field `DRSYNCR` writer - DRSYNCR
pub type DRSYNCR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LMPDRI` reader - LMPDRI
pub type LMPDRI_R = crate::FieldReader;
///Field `LMPDRI` writer - LMPDRI
pub type LMPDRI_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - LSI
    #[inline(always)]
    pub fn lsi(&self) -> LSI_R {
        LSI_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:10 - DRSYNCR
    #[inline(always)]
    pub fn drsyncr(&self) -> DRSYNCR_R {
        DRSYNCR_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 24:31 - LMPDRI
    #[inline(always)]
    pub fn lmpdri(&self) -> LMPDRI_R {
        LMPDRI_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETH_MACLMIR")
            .field("lsi", &self.lsi())
            .field("drsyncr", &self.drsyncr())
            .field("lmpdri", &self.lmpdri())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - LSI
    #[inline(always)]
    #[must_use]
    pub fn lsi(&mut self) -> LSI_W<ETH_MACLMIRrs> {
        LSI_W::new(self, 0)
    }
    ///Bits 8:10 - DRSYNCR
    #[inline(always)]
    #[must_use]
    pub fn drsyncr(&mut self) -> DRSYNCR_W<ETH_MACLMIRrs> {
        DRSYNCR_W::new(self, 8)
    }
    ///Bits 24:31 - LMPDRI
    #[inline(always)]
    #[must_use]
    pub fn lmpdri(&mut self) -> LMPDRI_W<ETH_MACLMIRrs> {
        LMPDRI_W::new(self, 24)
    }
}
/**This register contains the periodic intervals for automatic PTP packet generation.

You can [`read`](crate::Reg::read) this register and get [`eth_maclmir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_maclmir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MAC_MMC:ETH_MACLMIR)*/
pub struct ETH_MACLMIRrs;
impl crate::RegisterSpec for ETH_MACLMIRrs {
    type Ux = u32;
}
///`read()` method returns [`eth_maclmir::R`](R) reader structure
impl crate::Readable for ETH_MACLMIRrs {}
///`write(|w| ..)` method takes [`eth_maclmir::W`](W) writer structure
impl crate::Writable for ETH_MACLMIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ETH_MACLMIR to value 0
impl crate::Resettable for ETH_MACLMIRrs {
    const RESET_VALUE: u32 = 0;
}
