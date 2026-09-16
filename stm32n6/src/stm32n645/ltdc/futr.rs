///Register `FUTR` reader
pub type R = crate::R<FUTRrs>;
///Register `FUTR` writer
pub type W = crate::W<FUTRrs>;
///Field `THRE` reader - threshold to trigger a FIFO underrun interrupt (per FIFO word, 64 bits)
pub type THRE_R = crate::FieldReader<u16>;
///Field `THRE` writer - threshold to trigger a FIFO underrun interrupt (per FIFO word, 64 bits)
pub type THRE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - threshold to trigger a FIFO underrun interrupt (per FIFO word, 64 bits)
    #[inline(always)]
    pub fn thre(&self) -> THRE_R {
        THRE_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUTR").field("thre", &self.thre()).finish()
    }
}
impl W {
    ///Bits 0:15 - threshold to trigger a FIFO underrun interrupt (per FIFO word, 64 bits)
    #[inline(always)]
    pub fn thre(&mut self) -> THRE_W<'_, FUTRrs> {
        THRE_W::new(self, 0)
    }
}
/**LTDC FIFO underrun threshold register

You can [`read`](crate::Reg::read) this register and get [`futr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`futr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:FUTR)*/
pub struct FUTRrs;
impl crate::RegisterSpec for FUTRrs {
    type Ux = u32;
}
///`read()` method returns [`futr::R`](R) reader structure
impl crate::Readable for FUTRrs {}
///`write(|w| ..)` method takes [`futr::W`](W) writer structure
impl crate::Writable for FUTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FUTR to value 0x10
impl crate::Resettable for FUTRrs {
    const RESET_VALUE: u32 = 0x10;
}
