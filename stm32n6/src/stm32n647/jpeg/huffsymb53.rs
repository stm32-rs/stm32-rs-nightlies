///Register `HUFFSYMB53` reader
pub type R = crate::R<HUFFSYMB53rs>;
///Register `HUFFSYMB53` writer
pub type W = crate::W<HUFFSYMB53rs>;
///Field `DATA212` reader - Data 212
pub type DATA212_R = crate::FieldReader;
///Field `DATA212` writer - Data 212
pub type DATA212_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA213` reader - Data 213
pub type DATA213_R = crate::FieldReader;
///Field `DATA213` writer - Data 213
pub type DATA213_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA214` reader - Data 214
pub type DATA214_R = crate::FieldReader;
///Field `DATA214` writer - Data 214
pub type DATA214_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA215` reader - Data 215
pub type DATA215_R = crate::FieldReader;
///Field `DATA215` writer - Data 215
pub type DATA215_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 212
    #[inline(always)]
    pub fn data212(&self) -> DATA212_R {
        DATA212_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 213
    #[inline(always)]
    pub fn data213(&self) -> DATA213_R {
        DATA213_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 214
    #[inline(always)]
    pub fn data214(&self) -> DATA214_R {
        DATA214_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 215
    #[inline(always)]
    pub fn data215(&self) -> DATA215_R {
        DATA215_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB53")
            .field("data212", &self.data212())
            .field("data213", &self.data213())
            .field("data214", &self.data214())
            .field("data215", &self.data215())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 212
    #[inline(always)]
    pub fn data212(&mut self) -> DATA212_W<'_, HUFFSYMB53rs> {
        DATA212_W::new(self, 0)
    }
    ///Bits 8:15 - Data 213
    #[inline(always)]
    pub fn data213(&mut self) -> DATA213_W<'_, HUFFSYMB53rs> {
        DATA213_W::new(self, 8)
    }
    ///Bits 16:23 - Data 214
    #[inline(always)]
    pub fn data214(&mut self) -> DATA214_W<'_, HUFFSYMB53rs> {
        DATA214_W::new(self, 16)
    }
    ///Bits 24:31 - Data 215
    #[inline(always)]
    pub fn data215(&mut self) -> DATA215_W<'_, HUFFSYMB53rs> {
        DATA215_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb53::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb53::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFSYMB53)*/
pub struct HUFFSYMB53rs;
impl crate::RegisterSpec for HUFFSYMB53rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb53::R`](R) reader structure
impl crate::Readable for HUFFSYMB53rs {}
///`write(|w| ..)` method takes [`huffsymb53::W`](W) writer structure
impl crate::Writable for HUFFSYMB53rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB53 to value 0
impl crate::Resettable for HUFFSYMB53rs {}
