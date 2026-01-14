///Register `PAGEPROT1` reader
pub type R = crate::R<PAGEPROT1rs>;
///Register `PAGEPROT1` writer
pub type W = crate::W<PAGEPROT1rs>;
///Field `SEG2` reader - Third segment definition. See PAGEPROT0 SEG0 description for details on SEG2\[15:0\] content
pub type SEG2_R = crate::FieldReader<u16>;
///Field `SEG2` writer - Third segment definition. See PAGEPROT0 SEG0 description for details on SEG2\[15:0\] content
pub type SEG2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `SEG3` reader - Fourth segment definition. See PAGEPROT0 SEG0 description for details on SEG3\[15:0\] content.
pub type SEG3_R = crate::FieldReader<u16>;
///Field `SEG3` writer - Fourth segment definition. See PAGEPROT0 SEG0 description for details on SEG3\[15:0\] content.
pub type SEG3_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Third segment definition. See PAGEPROT0 SEG0 description for details on SEG2\[15:0\] content
    #[inline(always)]
    pub fn seg2(&self) -> SEG2_R {
        SEG2_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Fourth segment definition. See PAGEPROT0 SEG0 description for details on SEG3\[15:0\] content.
    #[inline(always)]
    pub fn seg3(&self) -> SEG3_R {
        SEG3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAGEPROT1")
            .field("seg2", &self.seg2())
            .field("seg3", &self.seg3())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Third segment definition. See PAGEPROT0 SEG0 description for details on SEG2\[15:0\] content
    #[inline(always)]
    pub fn seg2(&mut self) -> SEG2_W<'_, PAGEPROT1rs> {
        SEG2_W::new(self, 0)
    }
    ///Bits 16:31 - Fourth segment definition. See PAGEPROT0 SEG0 description for details on SEG3\[15:0\] content.
    #[inline(always)]
    pub fn seg3(&mut self) -> SEG3_W<'_, PAGEPROT1rs> {
        SEG3_W::new(self, 16)
    }
}
/**PAGEPROT1 register

You can [`read`](crate::Reg::read) this register and get [`pageprot1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pageprot1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#FLASH_CTRL:PAGEPROT1)*/
pub struct PAGEPROT1rs;
impl crate::RegisterSpec for PAGEPROT1rs {
    type Ux = u32;
}
///`read()` method returns [`pageprot1::R`](R) reader structure
impl crate::Readable for PAGEPROT1rs {}
///`write(|w| ..)` method takes [`pageprot1::W`](W) writer structure
impl crate::Writable for PAGEPROT1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PAGEPROT1 to value 0
impl crate::Resettable for PAGEPROT1rs {}
