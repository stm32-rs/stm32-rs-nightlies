///Register `DMAA4TxACR` reader
pub type R = crate::R<DMAA4TX_ACRrs>;
///Register `DMAA4TxACR` writer
pub type W = crate::W<DMAA4TX_ACRrs>;
///Field `TDRC` reader - TDRC
pub type TDRC_R = crate::FieldReader;
///Field `TDRC` writer - TDRC
pub type TDRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TEC` reader - TEC
pub type TEC_R = crate::FieldReader;
///Field `TEC` writer - TEC
pub type TEC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `THC` reader - THC
pub type THC_R = crate::FieldReader;
///Field `THC` writer - THC
pub type THC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - TDRC
    #[inline(always)]
    pub fn tdrc(&self) -> TDRC_R {
        TDRC_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:11 - TEC
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:19 - THC
    #[inline(always)]
    pub fn thc(&self) -> THC_R {
        THC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAA4TxACR")
            .field("tdrc", &self.tdrc())
            .field("tec", &self.tec())
            .field("thc", &self.thc())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - TDRC
    #[inline(always)]
    pub fn tdrc(&mut self) -> TDRC_W<'_, DMAA4TX_ACRrs> {
        TDRC_W::new(self, 0)
    }
    ///Bits 8:11 - TEC
    #[inline(always)]
    pub fn tec(&mut self) -> TEC_W<'_, DMAA4TX_ACRrs> {
        TEC_W::new(self, 8)
    }
    ///Bits 16:19 - THC
    #[inline(always)]
    pub fn thc(&mut self) -> THC_W<'_, DMAA4TX_ACRrs> {
        THC_W::new(self, 16)
    }
}
/**AXI4 transmit channel ACE control register

You can [`read`](crate::Reg::read) this register and get [`dmaa4tx_acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaa4tx_acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_DMA:DMAA4TxACR)*/
pub struct DMAA4TX_ACRrs;
impl crate::RegisterSpec for DMAA4TX_ACRrs {
    type Ux = u32;
}
///`read()` method returns [`dmaa4tx_acr::R`](R) reader structure
impl crate::Readable for DMAA4TX_ACRrs {}
///`write(|w| ..)` method takes [`dmaa4tx_acr::W`](W) writer structure
impl crate::Writable for DMAA4TX_ACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAA4TxACR to value 0
impl crate::Resettable for DMAA4TX_ACRrs {}
