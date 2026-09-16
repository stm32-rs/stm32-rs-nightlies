///Register `CONFR3` reader
pub type R = crate::R<CONFR3rs>;
///Register `CONFR3` writer
pub type W = crate::W<CONFR3rs>;
///Field `XSIZE` reader - X size This field defines the number of pixels per line.
pub type XSIZE_R = crate::FieldReader<u16>;
///Field `XSIZE` writer - X size This field defines the number of pixels per line.
pub type XSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 16:31 - X size This field defines the number of pixels per line.
    #[inline(always)]
    pub fn xsize(&self) -> XSIZE_R {
        XSIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFR3")
            .field("xsize", &self.xsize())
            .finish()
    }
}
impl W {
    ///Bits 16:31 - X size This field defines the number of pixels per line.
    #[inline(always)]
    pub fn xsize(&mut self) -> XSIZE_W<'_, CONFR3rs> {
        XSIZE_W::new(self, 16)
    }
}
/**JPEG codec configuration register 3

You can [`read`](crate::Reg::read) this register and get [`confr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#JPEG:CONFR3)*/
pub struct CONFR3rs;
impl crate::RegisterSpec for CONFR3rs {
    type Ux = u32;
}
///`read()` method returns [`confr3::R`](R) reader structure
impl crate::Readable for CONFR3rs {}
///`write(|w| ..)` method takes [`confr3::W`](W) writer structure
impl crate::Writable for CONFR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CONFR3 to value 0
impl crate::Resettable for CONFR3rs {}
