///Register `L1CFBLR` reader
pub type R = crate::R<L1CFBLRrs>;
///Register `L1CFBLR` writer
pub type W = crate::W<L1CFBLRrs>;
///Field `CFBLL` reader - color frame buffer line length These bits define the length of one line of pixels in bytes + 3. The line length is computed as follows: active high width * number of bytes per pixel + 3.
pub type CFBLL_R = crate::FieldReader<u16>;
///Field `CFBLL` writer - color frame buffer line length These bits define the length of one line of pixels in bytes + 3. The line length is computed as follows: active high width * number of bytes per pixel + 3.
pub type CFBLL_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `CFBP` reader - color frame buffer pitch in bytes These bits define the pitch that is the increment from the start of one line of pixels to the start of the next line in bytes.
pub type CFBP_R = crate::FieldReader<u16>;
///Field `CFBP` writer - color frame buffer pitch in bytes These bits define the pitch that is the increment from the start of one line of pixels to the start of the next line in bytes.
pub type CFBP_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:12 - color frame buffer line length These bits define the length of one line of pixels in bytes + 3. The line length is computed as follows: active high width * number of bytes per pixel + 3.
    #[inline(always)]
    pub fn cfbll(&self) -> CFBLL_R {
        CFBLL_R::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 16:28 - color frame buffer pitch in bytes These bits define the pitch that is the increment from the start of one line of pixels to the start of the next line in bytes.
    #[inline(always)]
    pub fn cfbp(&self) -> CFBP_R {
        CFBP_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1CFBLR")
            .field("cfbll", &self.cfbll())
            .field("cfbp", &self.cfbp())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - color frame buffer line length These bits define the length of one line of pixels in bytes + 3. The line length is computed as follows: active high width * number of bytes per pixel + 3.
    #[inline(always)]
    pub fn cfbll(&mut self) -> CFBLL_W<L1CFBLRrs> {
        CFBLL_W::new(self, 0)
    }
    ///Bits 16:28 - color frame buffer pitch in bytes These bits define the pitch that is the increment from the start of one line of pixels to the start of the next line in bytes.
    #[inline(always)]
    pub fn cfbp(&mut self) -> CFBP_W<L1CFBLRrs> {
        CFBP_W::new(self, 16)
    }
}
/**LTDC layer 1 color frame buffer length register

You can [`read`](crate::Reg::read) this register and get [`l1cfblr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1cfblr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#LTDC:L1CFBLR)*/
pub struct L1CFBLRrs;
impl crate::RegisterSpec for L1CFBLRrs {
    type Ux = u32;
}
///`read()` method returns [`l1cfblr::R`](R) reader structure
impl crate::Readable for L1CFBLRrs {}
///`write(|w| ..)` method takes [`l1cfblr::W`](W) writer structure
impl crate::Writable for L1CFBLRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L1CFBLR to value 0
impl crate::Resettable for L1CFBLRrs {}
