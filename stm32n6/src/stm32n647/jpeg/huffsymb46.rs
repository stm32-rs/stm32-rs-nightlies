///Register `HUFFSYMB46` reader
pub type R = crate::R<HUFFSYMB46rs>;
///Register `HUFFSYMB46` writer
pub type W = crate::W<HUFFSYMB46rs>;
///Field `DATA184` reader - Data 184
pub type DATA184_R = crate::FieldReader;
///Field `DATA184` writer - Data 184
pub type DATA184_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA185` reader - Data 185
pub type DATA185_R = crate::FieldReader;
///Field `DATA185` writer - Data 185
pub type DATA185_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA186` reader - Data 186
pub type DATA186_R = crate::FieldReader;
///Field `DATA186` writer - Data 186
pub type DATA186_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA187` reader - Data 187
pub type DATA187_R = crate::FieldReader;
///Field `DATA187` writer - Data 187
pub type DATA187_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 184
    #[inline(always)]
    pub fn data184(&self) -> DATA184_R {
        DATA184_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 185
    #[inline(always)]
    pub fn data185(&self) -> DATA185_R {
        DATA185_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 186
    #[inline(always)]
    pub fn data186(&self) -> DATA186_R {
        DATA186_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 187
    #[inline(always)]
    pub fn data187(&self) -> DATA187_R {
        DATA187_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB46")
            .field("data184", &self.data184())
            .field("data185", &self.data185())
            .field("data186", &self.data186())
            .field("data187", &self.data187())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 184
    #[inline(always)]
    pub fn data184(&mut self) -> DATA184_W<'_, HUFFSYMB46rs> {
        DATA184_W::new(self, 0)
    }
    ///Bits 8:15 - Data 185
    #[inline(always)]
    pub fn data185(&mut self) -> DATA185_W<'_, HUFFSYMB46rs> {
        DATA185_W::new(self, 8)
    }
    ///Bits 16:23 - Data 186
    #[inline(always)]
    pub fn data186(&mut self) -> DATA186_W<'_, HUFFSYMB46rs> {
        DATA186_W::new(self, 16)
    }
    ///Bits 24:31 - Data 187
    #[inline(always)]
    pub fn data187(&mut self) -> DATA187_W<'_, HUFFSYMB46rs> {
        DATA187_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb46::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb46::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFSYMB46)*/
pub struct HUFFSYMB46rs;
impl crate::RegisterSpec for HUFFSYMB46rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb46::R`](R) reader structure
impl crate::Readable for HUFFSYMB46rs {}
///`write(|w| ..)` method takes [`huffsymb46::W`](W) writer structure
impl crate::Writable for HUFFSYMB46rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB46 to value 0
impl crate::Resettable for HUFFSYMB46rs {}
