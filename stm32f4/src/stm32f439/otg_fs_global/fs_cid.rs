///Register `FS_CID` reader
pub type R = crate::R<FS_CIDrs>;
///Register `FS_CID` writer
pub type W = crate::W<FS_CIDrs>;
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
        f.debug_struct("FS_CID")
            .field("product_id", &self.product_id())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Product ID field
    #[inline(always)]
    pub fn product_id(&mut self) -> PRODUCT_ID_W<'_, FS_CIDrs> {
        PRODUCT_ID_W::new(self, 0)
    }
}
/**core ID register

You can [`read`](crate::Reg::read) this register and get [`fs_cid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_cid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#OTG_FS_GLOBAL:FS_CID)*/
pub struct FS_CIDrs;
impl crate::RegisterSpec for FS_CIDrs {
    type Ux = u32;
}
///`read()` method returns [`fs_cid::R`](R) reader structure
impl crate::Readable for FS_CIDrs {}
///`write(|w| ..)` method takes [`fs_cid::W`](W) writer structure
impl crate::Writable for FS_CIDrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FS_CID to value 0x1000
impl crate::Resettable for FS_CIDrs {
    const RESET_VALUE: u32 = 0x1000;
}
