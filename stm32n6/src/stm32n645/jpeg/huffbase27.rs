///Register `HUFFBASE27` reader
pub type R = crate::R<HUFFBASE27rs>;
///Register `HUFFBASE27` writer
pub type W = crate::W<HUFFBASE27rs>;
///Field `DATA54` reader - Data 54
pub type DATA54_R = crate::FieldReader<u16>;
///Field `DATA54` writer - Data 54
pub type DATA54_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA55` reader - Data 55
pub type DATA55_R = crate::FieldReader<u16>;
///Field `DATA55` writer - Data 55
pub type DATA55_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 54
    #[inline(always)]
    pub fn data54(&self) -> DATA54_R {
        DATA54_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 55
    #[inline(always)]
    pub fn data55(&self) -> DATA55_R {
        DATA55_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE27")
            .field("data54", &self.data54())
            .field("data55", &self.data55())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 54
    #[inline(always)]
    pub fn data54(&mut self) -> DATA54_W<'_, HUFFBASE27rs> {
        DATA54_W::new(self, 0)
    }
    ///Bits 16:24 - Data 55
    #[inline(always)]
    pub fn data55(&mut self) -> DATA55_W<'_, HUFFBASE27rs> {
        DATA55_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase27::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase27::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFBASE27)*/
pub struct HUFFBASE27rs;
impl crate::RegisterSpec for HUFFBASE27rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase27::R`](R) reader structure
impl crate::Readable for HUFFBASE27rs {}
///`write(|w| ..)` method takes [`huffbase27::W`](W) writer structure
impl crate::Writable for HUFFBASE27rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE27 to value 0
impl crate::Resettable for HUFFBASE27rs {}
