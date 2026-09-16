///Register `HUFFSYMB38` reader
pub type R = crate::R<HUFFSYMB38rs>;
///Register `HUFFSYMB38` writer
pub type W = crate::W<HUFFSYMB38rs>;
///Field `DATA152` reader - Data 152
pub type DATA152_R = crate::FieldReader;
///Field `DATA152` writer - Data 152
pub type DATA152_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA153` reader - Data 153
pub type DATA153_R = crate::FieldReader;
///Field `DATA153` writer - Data 153
pub type DATA153_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA154` reader - Data 154
pub type DATA154_R = crate::FieldReader;
///Field `DATA154` writer - Data 154
pub type DATA154_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA155` reader - Data 155
pub type DATA155_R = crate::FieldReader;
///Field `DATA155` writer - Data 155
pub type DATA155_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 152
    #[inline(always)]
    pub fn data152(&self) -> DATA152_R {
        DATA152_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 153
    #[inline(always)]
    pub fn data153(&self) -> DATA153_R {
        DATA153_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 154
    #[inline(always)]
    pub fn data154(&self) -> DATA154_R {
        DATA154_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 155
    #[inline(always)]
    pub fn data155(&self) -> DATA155_R {
        DATA155_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB38")
            .field("data152", &self.data152())
            .field("data153", &self.data153())
            .field("data154", &self.data154())
            .field("data155", &self.data155())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 152
    #[inline(always)]
    pub fn data152(&mut self) -> DATA152_W<'_, HUFFSYMB38rs> {
        DATA152_W::new(self, 0)
    }
    ///Bits 8:15 - Data 153
    #[inline(always)]
    pub fn data153(&mut self) -> DATA153_W<'_, HUFFSYMB38rs> {
        DATA153_W::new(self, 8)
    }
    ///Bits 16:23 - Data 154
    #[inline(always)]
    pub fn data154(&mut self) -> DATA154_W<'_, HUFFSYMB38rs> {
        DATA154_W::new(self, 16)
    }
    ///Bits 24:31 - Data 155
    #[inline(always)]
    pub fn data155(&mut self) -> DATA155_W<'_, HUFFSYMB38rs> {
        DATA155_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb38::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb38::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFSYMB38)*/
pub struct HUFFSYMB38rs;
impl crate::RegisterSpec for HUFFSYMB38rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb38::R`](R) reader structure
impl crate::Readable for HUFFSYMB38rs {}
///`write(|w| ..)` method takes [`huffsymb38::W`](W) writer structure
impl crate::Writable for HUFFSYMB38rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB38 to value 0
impl crate::Resettable for HUFFSYMB38rs {}
