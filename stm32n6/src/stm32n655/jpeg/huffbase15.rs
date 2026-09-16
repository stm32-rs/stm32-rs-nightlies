///Register `HUFFBASE15` reader
pub type R = crate::R<HUFFBASE15rs>;
///Register `HUFFBASE15` writer
pub type W = crate::W<HUFFBASE15rs>;
///Field `DATA30` reader - Data 30
pub type DATA30_R = crate::FieldReader<u16>;
///Field `DATA30` writer - Data 30
pub type DATA30_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA31` reader - Data 31
pub type DATA31_R = crate::FieldReader<u16>;
///Field `DATA31` writer - Data 31
pub type DATA31_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 30
    #[inline(always)]
    pub fn data30(&self) -> DATA30_R {
        DATA30_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 31
    #[inline(always)]
    pub fn data31(&self) -> DATA31_R {
        DATA31_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE15")
            .field("data30", &self.data30())
            .field("data31", &self.data31())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 30
    #[inline(always)]
    pub fn data30(&mut self) -> DATA30_W<'_, HUFFBASE15rs> {
        DATA30_W::new(self, 0)
    }
    ///Bits 16:24 - Data 31
    #[inline(always)]
    pub fn data31(&mut self) -> DATA31_W<'_, HUFFBASE15rs> {
        DATA31_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE15)*/
pub struct HUFFBASE15rs;
impl crate::RegisterSpec for HUFFBASE15rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase15::R`](R) reader structure
impl crate::Readable for HUFFBASE15rs {}
///`write(|w| ..)` method takes [`huffbase15::W`](W) writer structure
impl crate::Writable for HUFFBASE15rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE15 to value 0
impl crate::Resettable for HUFFBASE15rs {}
