///Register `HUFFSYMB7` reader
pub type R = crate::R<HUFFSYMB7rs>;
///Register `HUFFSYMB7` writer
pub type W = crate::W<HUFFSYMB7rs>;
///Field `DATA28` reader - Data 28
pub type DATA28_R = crate::FieldReader;
///Field `DATA28` writer - Data 28
pub type DATA28_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA29` reader - Data 29
pub type DATA29_R = crate::FieldReader;
///Field `DATA29` writer - Data 29
pub type DATA29_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA30` reader - Data 30
pub type DATA30_R = crate::FieldReader;
///Field `DATA30` writer - Data 30
pub type DATA30_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA31` reader - Data 31
pub type DATA31_R = crate::FieldReader;
///Field `DATA31` writer - Data 31
pub type DATA31_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 28
    #[inline(always)]
    pub fn data28(&self) -> DATA28_R {
        DATA28_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 29
    #[inline(always)]
    pub fn data29(&self) -> DATA29_R {
        DATA29_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 30
    #[inline(always)]
    pub fn data30(&self) -> DATA30_R {
        DATA30_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 31
    #[inline(always)]
    pub fn data31(&self) -> DATA31_R {
        DATA31_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB7")
            .field("data28", &self.data28())
            .field("data29", &self.data29())
            .field("data30", &self.data30())
            .field("data31", &self.data31())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 28
    #[inline(always)]
    pub fn data28(&mut self) -> DATA28_W<'_, HUFFSYMB7rs> {
        DATA28_W::new(self, 0)
    }
    ///Bits 8:15 - Data 29
    #[inline(always)]
    pub fn data29(&mut self) -> DATA29_W<'_, HUFFSYMB7rs> {
        DATA29_W::new(self, 8)
    }
    ///Bits 16:23 - Data 30
    #[inline(always)]
    pub fn data30(&mut self) -> DATA30_W<'_, HUFFSYMB7rs> {
        DATA30_W::new(self, 16)
    }
    ///Bits 24:31 - Data 31
    #[inline(always)]
    pub fn data31(&mut self) -> DATA31_W<'_, HUFFSYMB7rs> {
        DATA31_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB7)*/
pub struct HUFFSYMB7rs;
impl crate::RegisterSpec for HUFFSYMB7rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb7::R`](R) reader structure
impl crate::Readable for HUFFSYMB7rs {}
///`write(|w| ..)` method takes [`huffsymb7::W`](W) writer structure
impl crate::Writable for HUFFSYMB7rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB7 to value 0
impl crate::Resettable for HUFFSYMB7rs {}
