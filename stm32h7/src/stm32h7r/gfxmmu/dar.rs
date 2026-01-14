///Register `DAR` reader
pub type R = crate::R<DARrs>;
///Register `DAR` writer
pub type W = crate::W<DARrs>;
///Field `DA` reader - Default alpha This field indicates the default 8-bit value which is merged with the 24-bit value when a master accesses a virtual memory location in packed mode.
pub type DA_R = crate::FieldReader;
///Field `DA` writer - Default alpha This field indicates the default 8-bit value which is merged with the 24-bit value when a master accesses a virtual memory location in packed mode.
pub type DA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Default alpha This field indicates the default 8-bit value which is merged with the 24-bit value when a master accesses a virtual memory location in packed mode.
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAR").field("da", &self.da()).finish()
    }
}
impl W {
    ///Bits 0:7 - Default alpha This field indicates the default 8-bit value which is merged with the 24-bit value when a master accesses a virtual memory location in packed mode.
    #[inline(always)]
    pub fn da(&mut self) -> DA_W<'_, DARrs> {
        DA_W::new(self, 0)
    }
}
/**GFXMMU default alpha register

You can [`read`](crate::Reg::read) this register and get [`dar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#GFXMMU:DAR)*/
pub struct DARrs;
impl crate::RegisterSpec for DARrs {
    type Ux = u32;
}
///`read()` method returns [`dar::R`](R) reader structure
impl crate::Readable for DARrs {}
///`write(|w| ..)` method takes [`dar::W`](W) writer structure
impl crate::Writable for DARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DAR to value 0
impl crate::Resettable for DARrs {}
