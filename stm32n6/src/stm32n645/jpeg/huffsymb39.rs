///Register `HUFFSYMB39` reader
pub type R = crate::R<HUFFSYMB39rs>;
///Register `HUFFSYMB39` writer
pub type W = crate::W<HUFFSYMB39rs>;
///Field `DATA156` reader - Data 156
pub type DATA156_R = crate::FieldReader;
///Field `DATA156` writer - Data 156
pub type DATA156_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA157` reader - Data 157
pub type DATA157_R = crate::FieldReader;
///Field `DATA157` writer - Data 157
pub type DATA157_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA158` reader - Data 158
pub type DATA158_R = crate::FieldReader;
///Field `DATA158` writer - Data 158
pub type DATA158_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA159` reader - Data 159
pub type DATA159_R = crate::FieldReader;
///Field `DATA159` writer - Data 159
pub type DATA159_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 156
    #[inline(always)]
    pub fn data156(&self) -> DATA156_R {
        DATA156_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 157
    #[inline(always)]
    pub fn data157(&self) -> DATA157_R {
        DATA157_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 158
    #[inline(always)]
    pub fn data158(&self) -> DATA158_R {
        DATA158_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 159
    #[inline(always)]
    pub fn data159(&self) -> DATA159_R {
        DATA159_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB39")
            .field("data156", &self.data156())
            .field("data157", &self.data157())
            .field("data158", &self.data158())
            .field("data159", &self.data159())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 156
    #[inline(always)]
    pub fn data156(&mut self) -> DATA156_W<'_, HUFFSYMB39rs> {
        DATA156_W::new(self, 0)
    }
    ///Bits 8:15 - Data 157
    #[inline(always)]
    pub fn data157(&mut self) -> DATA157_W<'_, HUFFSYMB39rs> {
        DATA157_W::new(self, 8)
    }
    ///Bits 16:23 - Data 158
    #[inline(always)]
    pub fn data158(&mut self) -> DATA158_W<'_, HUFFSYMB39rs> {
        DATA158_W::new(self, 16)
    }
    ///Bits 24:31 - Data 159
    #[inline(always)]
    pub fn data159(&mut self) -> DATA159_W<'_, HUFFSYMB39rs> {
        DATA159_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb39::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb39::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFSYMB39)*/
pub struct HUFFSYMB39rs;
impl crate::RegisterSpec for HUFFSYMB39rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb39::R`](R) reader structure
impl crate::Readable for HUFFSYMB39rs {}
///`write(|w| ..)` method takes [`huffsymb39::W`](W) writer structure
impl crate::Writable for HUFFSYMB39rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB39 to value 0
impl crate::Resettable for HUFFSYMB39rs {}
