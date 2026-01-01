///Register `HUFFSYMB20` reader
pub type R = crate::R<HUFFSYMB20rs>;
///Register `HUFFSYMB20` writer
pub type W = crate::W<HUFFSYMB20rs>;
///Field `DATA80` reader - Data 80
pub type DATA80_R = crate::FieldReader;
///Field `DATA80` writer - Data 80
pub type DATA80_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA81` reader - Data 81
pub type DATA81_R = crate::FieldReader;
///Field `DATA81` writer - Data 81
pub type DATA81_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA82` reader - Data 82
pub type DATA82_R = crate::FieldReader;
///Field `DATA82` writer - Data 82
pub type DATA82_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA83` reader - Data 83
pub type DATA83_R = crate::FieldReader;
///Field `DATA83` writer - Data 83
pub type DATA83_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 80
    #[inline(always)]
    pub fn data80(&self) -> DATA80_R {
        DATA80_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 81
    #[inline(always)]
    pub fn data81(&self) -> DATA81_R {
        DATA81_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 82
    #[inline(always)]
    pub fn data82(&self) -> DATA82_R {
        DATA82_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 83
    #[inline(always)]
    pub fn data83(&self) -> DATA83_R {
        DATA83_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB20")
            .field("data80", &self.data80())
            .field("data81", &self.data81())
            .field("data82", &self.data82())
            .field("data83", &self.data83())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 80
    #[inline(always)]
    pub fn data80(&mut self) -> DATA80_W<'_, HUFFSYMB20rs> {
        DATA80_W::new(self, 0)
    }
    ///Bits 8:15 - Data 81
    #[inline(always)]
    pub fn data81(&mut self) -> DATA81_W<'_, HUFFSYMB20rs> {
        DATA81_W::new(self, 8)
    }
    ///Bits 16:23 - Data 82
    #[inline(always)]
    pub fn data82(&mut self) -> DATA82_W<'_, HUFFSYMB20rs> {
        DATA82_W::new(self, 16)
    }
    ///Bits 24:31 - Data 83
    #[inline(always)]
    pub fn data83(&mut self) -> DATA83_W<'_, HUFFSYMB20rs> {
        DATA83_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB20)*/
pub struct HUFFSYMB20rs;
impl crate::RegisterSpec for HUFFSYMB20rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb20::R`](R) reader structure
impl crate::Readable for HUFFSYMB20rs {}
///`write(|w| ..)` method takes [`huffsymb20::W`](W) writer structure
impl crate::Writable for HUFFSYMB20rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB20 to value 0
impl crate::Resettable for HUFFSYMB20rs {}
