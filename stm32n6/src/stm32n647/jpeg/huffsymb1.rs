///Register `HUFFSYMB1` reader
pub type R = crate::R<HUFFSYMB1rs>;
///Register `HUFFSYMB1` writer
pub type W = crate::W<HUFFSYMB1rs>;
///Field `DATA4` reader - Data 4
pub type DATA4_R = crate::FieldReader;
///Field `DATA4` writer - Data 4
pub type DATA4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA5` reader - Data 5
pub type DATA5_R = crate::FieldReader;
///Field `DATA5` writer - Data 5
pub type DATA5_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA6` reader - Data 6
pub type DATA6_R = crate::FieldReader;
///Field `DATA6` writer - Data 6
pub type DATA6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA7` reader - Data 7
pub type DATA7_R = crate::FieldReader;
///Field `DATA7` writer - Data 7
pub type DATA7_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 4
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 5
    #[inline(always)]
    pub fn data5(&self) -> DATA5_R {
        DATA5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 6
    #[inline(always)]
    pub fn data6(&self) -> DATA6_R {
        DATA6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 7
    #[inline(always)]
    pub fn data7(&self) -> DATA7_R {
        DATA7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB1")
            .field("data4", &self.data4())
            .field("data5", &self.data5())
            .field("data6", &self.data6())
            .field("data7", &self.data7())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 4
    #[inline(always)]
    pub fn data4(&mut self) -> DATA4_W<'_, HUFFSYMB1rs> {
        DATA4_W::new(self, 0)
    }
    ///Bits 8:15 - Data 5
    #[inline(always)]
    pub fn data5(&mut self) -> DATA5_W<'_, HUFFSYMB1rs> {
        DATA5_W::new(self, 8)
    }
    ///Bits 16:23 - Data 6
    #[inline(always)]
    pub fn data6(&mut self) -> DATA6_W<'_, HUFFSYMB1rs> {
        DATA6_W::new(self, 16)
    }
    ///Bits 24:31 - Data 7
    #[inline(always)]
    pub fn data7(&mut self) -> DATA7_W<'_, HUFFSYMB1rs> {
        DATA7_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFSYMB1)*/
pub struct HUFFSYMB1rs;
impl crate::RegisterSpec for HUFFSYMB1rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb1::R`](R) reader structure
impl crate::Readable for HUFFSYMB1rs {}
///`write(|w| ..)` method takes [`huffsymb1::W`](W) writer structure
impl crate::Writable for HUFFSYMB1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB1 to value 0
impl crate::Resettable for HUFFSYMB1rs {}
