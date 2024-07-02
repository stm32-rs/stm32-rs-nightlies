///Register `LPDMA_C2DAR` reader
pub type R = crate::R<LPDMA_C2DARrs>;
///Register `LPDMA_C2DAR` writer
pub type W = crate::W<LPDMA_C2DARrs>;
///Field `DA` reader - destination address
pub type DA_R = crate::FieldReader<u32>;
///Field `DA` writer - destination address
pub type DA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - destination address
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPDMA_C2DAR")
            .field("da", &self.da())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - destination address
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DA_W<LPDMA_C2DARrs> {
        DA_W::new(self, 0)
    }
}
/**LPDMA channel x destination address register

You can [`read`](crate::Reg::read) this register and get [`lpdma_c2dar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpdma_c2dar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#LPDMA1:LPDMA_C2DAR)*/
pub struct LPDMA_C2DARrs;
impl crate::RegisterSpec for LPDMA_C2DARrs {
    type Ux = u32;
}
///`read()` method returns [`lpdma_c2dar::R`](R) reader structure
impl crate::Readable for LPDMA_C2DARrs {}
///`write(|w| ..)` method takes [`lpdma_c2dar::W`](W) writer structure
impl crate::Writable for LPDMA_C2DARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LPDMA_C2DAR to value 0
impl crate::Resettable for LPDMA_C2DARrs {
    const RESET_VALUE: u32 = 0;
}
