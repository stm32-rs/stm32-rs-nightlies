///Register `HUFFSYMB23` reader
pub type R = crate::R<HUFFSYMB23rs>;
///Register `HUFFSYMB23` writer
pub type W = crate::W<HUFFSYMB23rs>;
///Field `DATA92` reader - Data 92
pub type DATA92_R = crate::FieldReader;
///Field `DATA92` writer - Data 92
pub type DATA92_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA93` reader - Data 93
pub type DATA93_R = crate::FieldReader;
///Field `DATA93` writer - Data 93
pub type DATA93_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA94` reader - Data 94
pub type DATA94_R = crate::FieldReader;
///Field `DATA94` writer - Data 94
pub type DATA94_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA95` reader - Data 95
pub type DATA95_R = crate::FieldReader;
///Field `DATA95` writer - Data 95
pub type DATA95_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 92
    #[inline(always)]
    pub fn data92(&self) -> DATA92_R {
        DATA92_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 93
    #[inline(always)]
    pub fn data93(&self) -> DATA93_R {
        DATA93_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 94
    #[inline(always)]
    pub fn data94(&self) -> DATA94_R {
        DATA94_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 95
    #[inline(always)]
    pub fn data95(&self) -> DATA95_R {
        DATA95_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB23")
            .field("data92", &self.data92())
            .field("data93", &self.data93())
            .field("data94", &self.data94())
            .field("data95", &self.data95())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 92
    #[inline(always)]
    pub fn data92(&mut self) -> DATA92_W<'_, HUFFSYMB23rs> {
        DATA92_W::new(self, 0)
    }
    ///Bits 8:15 - Data 93
    #[inline(always)]
    pub fn data93(&mut self) -> DATA93_W<'_, HUFFSYMB23rs> {
        DATA93_W::new(self, 8)
    }
    ///Bits 16:23 - Data 94
    #[inline(always)]
    pub fn data94(&mut self) -> DATA94_W<'_, HUFFSYMB23rs> {
        DATA94_W::new(self, 16)
    }
    ///Bits 24:31 - Data 95
    #[inline(always)]
    pub fn data95(&mut self) -> DATA95_W<'_, HUFFSYMB23rs> {
        DATA95_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFSYMB23)*/
pub struct HUFFSYMB23rs;
impl crate::RegisterSpec for HUFFSYMB23rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb23::R`](R) reader structure
impl crate::Readable for HUFFSYMB23rs {}
///`write(|w| ..)` method takes [`huffsymb23::W`](W) writer structure
impl crate::Writable for HUFFSYMB23rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB23 to value 0
impl crate::Resettable for HUFFSYMB23rs {}
