///Register `PTACONV_CR` reader
pub type R = crate::R<PTACONV_CRrs>;
///Register `PTACONV_CR` writer
pub type W = crate::W<PTACONV_CRrs>;
///Field `TXRXPOL` reader - PTA_STATUS transmit and receive polarity.
pub type TXRXPOL_R = crate::BitReader;
///Field `TXRXPOL` writer - PTA_STATUS transmit and receive polarity.
pub type TXRXPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GRANTPOL` reader - PTA_GRANT polarity.
pub type GRANTPOL_R = crate::BitReader;
///Field `GRANTPOL` writer - PTA_GRANT polarity.
pub type GRANTPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 15 - PTA_STATUS transmit and receive polarity.
    #[inline(always)]
    pub fn txrxpol(&self) -> TXRXPOL_R {
        TXRXPOL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 31 - PTA_GRANT polarity.
    #[inline(always)]
    pub fn grantpol(&self) -> GRANTPOL_R {
        GRANTPOL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTACONV_CR")
            .field("txrxpol", &self.txrxpol())
            .field("grantpol", &self.grantpol())
            .finish()
    }
}
impl W {
    ///Bit 15 - PTA_STATUS transmit and receive polarity.
    #[inline(always)]
    pub fn txrxpol(&mut self) -> TXRXPOL_W<'_, PTACONV_CRrs> {
        TXRXPOL_W::new(self, 15)
    }
    ///Bit 31 - PTA_GRANT polarity.
    #[inline(always)]
    pub fn grantpol(&mut self) -> GRANTPOL_W<'_, PTACONV_CRrs> {
        GRANTPOL_W::new(self, 31)
    }
}
/**PTACONV control register

You can [`read`](crate::Reg::read) this register and get [`ptaconv_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptaconv_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#PTACONV:PTACONV_CR)*/
pub struct PTACONV_CRrs;
impl crate::RegisterSpec for PTACONV_CRrs {
    type Ux = u32;
}
///`read()` method returns [`ptaconv_cr::R`](R) reader structure
impl crate::Readable for PTACONV_CRrs {}
///`write(|w| ..)` method takes [`ptaconv_cr::W`](W) writer structure
impl crate::Writable for PTACONV_CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PTACONV_CR to value 0
impl crate::Resettable for PTACONV_CRrs {}
