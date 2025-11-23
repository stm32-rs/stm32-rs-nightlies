///Register `HUFFSYMB45` reader
pub type R = crate::R<HUFFSYMB45rs>;
///Register `HUFFSYMB45` writer
pub type W = crate::W<HUFFSYMB45rs>;
///Field `DATA180` reader - Data 180
pub type DATA180_R = crate::FieldReader;
///Field `DATA180` writer - Data 180
pub type DATA180_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA181` reader - Data 181
pub type DATA181_R = crate::FieldReader;
///Field `DATA181` writer - Data 181
pub type DATA181_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA182` reader - Data 182
pub type DATA182_R = crate::FieldReader;
///Field `DATA182` writer - Data 182
pub type DATA182_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA183` reader - Data 183
pub type DATA183_R = crate::FieldReader;
///Field `DATA183` writer - Data 183
pub type DATA183_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 180
    #[inline(always)]
    pub fn data180(&self) -> DATA180_R {
        DATA180_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 181
    #[inline(always)]
    pub fn data181(&self) -> DATA181_R {
        DATA181_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 182
    #[inline(always)]
    pub fn data182(&self) -> DATA182_R {
        DATA182_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 183
    #[inline(always)]
    pub fn data183(&self) -> DATA183_R {
        DATA183_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB45")
            .field("data180", &self.data180())
            .field("data181", &self.data181())
            .field("data182", &self.data182())
            .field("data183", &self.data183())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 180
    #[inline(always)]
    pub fn data180(&mut self) -> DATA180_W<'_, HUFFSYMB45rs> {
        DATA180_W::new(self, 0)
    }
    ///Bits 8:15 - Data 181
    #[inline(always)]
    pub fn data181(&mut self) -> DATA181_W<'_, HUFFSYMB45rs> {
        DATA181_W::new(self, 8)
    }
    ///Bits 16:23 - Data 182
    #[inline(always)]
    pub fn data182(&mut self) -> DATA182_W<'_, HUFFSYMB45rs> {
        DATA182_W::new(self, 16)
    }
    ///Bits 24:31 - Data 183
    #[inline(always)]
    pub fn data183(&mut self) -> DATA183_W<'_, HUFFSYMB45rs> {
        DATA183_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb45::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb45::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB45)*/
pub struct HUFFSYMB45rs;
impl crate::RegisterSpec for HUFFSYMB45rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb45::R`](R) reader structure
impl crate::Readable for HUFFSYMB45rs {}
///`write(|w| ..)` method takes [`huffsymb45::W`](W) writer structure
impl crate::Writable for HUFFSYMB45rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB45 to value 0
impl crate::Resettable for HUFFSYMB45rs {}
