///Register `HUFFBASE25` reader
pub type R = crate::R<HUFFBASE25rs>;
///Register `HUFFBASE25` writer
pub type W = crate::W<HUFFBASE25rs>;
///Field `DATA50` reader - Data 50
pub type DATA50_R = crate::FieldReader<u16>;
///Field `DATA50` writer - Data 50
pub type DATA50_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA51` reader - Data 51
pub type DATA51_R = crate::FieldReader<u16>;
///Field `DATA51` writer - Data 51
pub type DATA51_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 50
    #[inline(always)]
    pub fn data50(&self) -> DATA50_R {
        DATA50_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 51
    #[inline(always)]
    pub fn data51(&self) -> DATA51_R {
        DATA51_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE25")
            .field("data50", &self.data50())
            .field("data51", &self.data51())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 50
    #[inline(always)]
    pub fn data50(&mut self) -> DATA50_W<'_, HUFFBASE25rs> {
        DATA50_W::new(self, 0)
    }
    ///Bits 16:24 - Data 51
    #[inline(always)]
    pub fn data51(&mut self) -> DATA51_W<'_, HUFFBASE25rs> {
        DATA51_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase25::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase25::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFBASE25)*/
pub struct HUFFBASE25rs;
impl crate::RegisterSpec for HUFFBASE25rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase25::R`](R) reader structure
impl crate::Readable for HUFFBASE25rs {}
///`write(|w| ..)` method takes [`huffbase25::W`](W) writer structure
impl crate::Writable for HUFFBASE25rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE25 to value 0
impl crate::Resettable for HUFFBASE25rs {}
