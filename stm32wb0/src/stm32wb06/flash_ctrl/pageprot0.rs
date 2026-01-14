///Register `PAGEPROT0` reader
pub type R = crate::R<PAGEPROT0rs>;
///Register `PAGEPROT0` writer
pub type W = crate::W<PAGEPROT0rs>;
///Field `SEG0` reader - First segment definition.
pub type SEG0_R = crate::FieldReader<u16>;
///Field `SEG0` writer - First segment definition.
pub type SEG0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `SEG1` reader - Second segment definition. See SEG0 description for details on SEG1\[31:16\] content
pub type SEG1_R = crate::FieldReader<u16>;
///Field `SEG1` writer - Second segment definition. See SEG0 description for details on SEG1\[31:16\] content
pub type SEG1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - First segment definition.
    #[inline(always)]
    pub fn seg0(&self) -> SEG0_R {
        SEG0_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Second segment definition. See SEG0 description for details on SEG1\[31:16\] content
    #[inline(always)]
    pub fn seg1(&self) -> SEG1_R {
        SEG1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAGEPROT0")
            .field("seg0", &self.seg0())
            .field("seg1", &self.seg1())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - First segment definition.
    #[inline(always)]
    pub fn seg0(&mut self) -> SEG0_W<'_, PAGEPROT0rs> {
        SEG0_W::new(self, 0)
    }
    ///Bits 16:31 - Second segment definition. See SEG0 description for details on SEG1\[31:16\] content
    #[inline(always)]
    pub fn seg1(&mut self) -> SEG1_W<'_, PAGEPROT0rs> {
        SEG1_W::new(self, 16)
    }
}
/**PAGEPROT0 register

You can [`read`](crate::Reg::read) this register and get [`pageprot0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pageprot0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#FLASH_CTRL:PAGEPROT0)*/
pub struct PAGEPROT0rs;
impl crate::RegisterSpec for PAGEPROT0rs {
    type Ux = u32;
}
///`read()` method returns [`pageprot0::R`](R) reader structure
impl crate::Readable for PAGEPROT0rs {}
///`write(|w| ..)` method takes [`pageprot0::W`](W) writer structure
impl crate::Writable for PAGEPROT0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PAGEPROT0 to value 0
impl crate::Resettable for PAGEPROT0rs {}
