///Register `OTG_HS_CID` reader
pub type R = crate::R<OTG_HS_CIDrs>;
///Register `OTG_HS_CID` writer
pub type W = crate::W<OTG_HS_CIDrs>;
///Field `PRODUCT_ID` reader - Product ID field
pub type PRODUCT_ID_R = crate::FieldReader<u32>;
///Field `PRODUCT_ID` writer - Product ID field
pub type PRODUCT_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Product ID field
    #[inline(always)]
    pub fn product_id(&self) -> PRODUCT_ID_R {
        PRODUCT_ID_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_HS_CID")
            .field("product_id", &self.product_id())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Product ID field
    #[inline(always)]
    pub fn product_id(&mut self) -> PRODUCT_ID_W<'_, OTG_HS_CIDrs> {
        PRODUCT_ID_W::new(self, 0)
    }
}
/**OTG_HS core ID register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_cid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_cid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_HS_GLOBAL:OTG_HS_CID)*/
pub struct OTG_HS_CIDrs;
impl crate::RegisterSpec for OTG_HS_CIDrs {
    type Ux = u32;
}
///`read()` method returns [`otg_hs_cid::R`](R) reader structure
impl crate::Readable for OTG_HS_CIDrs {}
///`write(|w| ..)` method takes [`otg_hs_cid::W`](W) writer structure
impl crate::Writable for OTG_HS_CIDrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTG_HS_CID to value 0x1200
impl crate::Resettable for OTG_HS_CIDrs {
    const RESET_VALUE: u32 = 0x1200;
}
