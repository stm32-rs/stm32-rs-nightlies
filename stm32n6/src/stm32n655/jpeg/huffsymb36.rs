///Register `HUFFSYMB36` reader
pub type R = crate::R<HUFFSYMB36rs>;
///Register `HUFFSYMB36` writer
pub type W = crate::W<HUFFSYMB36rs>;
///Field `DATA144` reader - Data 144
pub type DATA144_R = crate::FieldReader;
///Field `DATA144` writer - Data 144
pub type DATA144_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA145` reader - Data 145
pub type DATA145_R = crate::FieldReader;
///Field `DATA145` writer - Data 145
pub type DATA145_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA146` reader - Data 146
pub type DATA146_R = crate::FieldReader;
///Field `DATA146` writer - Data 146
pub type DATA146_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA147` reader - Data 147
pub type DATA147_R = crate::FieldReader;
///Field `DATA147` writer - Data 147
pub type DATA147_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 144
    #[inline(always)]
    pub fn data144(&self) -> DATA144_R {
        DATA144_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 145
    #[inline(always)]
    pub fn data145(&self) -> DATA145_R {
        DATA145_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 146
    #[inline(always)]
    pub fn data146(&self) -> DATA146_R {
        DATA146_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 147
    #[inline(always)]
    pub fn data147(&self) -> DATA147_R {
        DATA147_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB36")
            .field("data144", &self.data144())
            .field("data145", &self.data145())
            .field("data146", &self.data146())
            .field("data147", &self.data147())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 144
    #[inline(always)]
    pub fn data144(&mut self) -> DATA144_W<'_, HUFFSYMB36rs> {
        DATA144_W::new(self, 0)
    }
    ///Bits 8:15 - Data 145
    #[inline(always)]
    pub fn data145(&mut self) -> DATA145_W<'_, HUFFSYMB36rs> {
        DATA145_W::new(self, 8)
    }
    ///Bits 16:23 - Data 146
    #[inline(always)]
    pub fn data146(&mut self) -> DATA146_W<'_, HUFFSYMB36rs> {
        DATA146_W::new(self, 16)
    }
    ///Bits 24:31 - Data 147
    #[inline(always)]
    pub fn data147(&mut self) -> DATA147_W<'_, HUFFSYMB36rs> {
        DATA147_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb36::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb36::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB36)*/
pub struct HUFFSYMB36rs;
impl crate::RegisterSpec for HUFFSYMB36rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb36::R`](R) reader structure
impl crate::Readable for HUFFSYMB36rs {}
///`write(|w| ..)` method takes [`huffsymb36::W`](W) writer structure
impl crate::Writable for HUFFSYMB36rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB36 to value 0
impl crate::Resettable for HUFFSYMB36rs {}
