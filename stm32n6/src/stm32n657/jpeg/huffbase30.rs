///Register `HUFFBASE30` reader
pub type R = crate::R<HUFFBASE30rs>;
///Register `HUFFBASE30` writer
pub type W = crate::W<HUFFBASE30rs>;
///Field `DATA60` reader - Data 60
pub type DATA60_R = crate::FieldReader<u16>;
///Field `DATA60` writer - Data 60
pub type DATA60_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA61` reader - Data 61
pub type DATA61_R = crate::FieldReader<u16>;
///Field `DATA61` writer - Data 61
pub type DATA61_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 60
    #[inline(always)]
    pub fn data60(&self) -> DATA60_R {
        DATA60_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 61
    #[inline(always)]
    pub fn data61(&self) -> DATA61_R {
        DATA61_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE30")
            .field("data60", &self.data60())
            .field("data61", &self.data61())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 60
    #[inline(always)]
    pub fn data60(&mut self) -> DATA60_W<'_, HUFFBASE30rs> {
        DATA60_W::new(self, 0)
    }
    ///Bits 16:24 - Data 61
    #[inline(always)]
    pub fn data61(&mut self) -> DATA61_W<'_, HUFFBASE30rs> {
        DATA61_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFBASE30)*/
pub struct HUFFBASE30rs;
impl crate::RegisterSpec for HUFFBASE30rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase30::R`](R) reader structure
impl crate::Readable for HUFFBASE30rs {}
///`write(|w| ..)` method takes [`huffbase30::W`](W) writer structure
impl crate::Writable for HUFFBASE30rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE30 to value 0
impl crate::Resettable for HUFFBASE30rs {}
