///Register `VDSL` reader
pub type R = crate::R<VDSLrs>;
///Register `VDSL` writer
pub type W = crate::W<VDSLrs>;
///Field `LENG` reader - Non-volatile data segment length
pub type LENG_R = crate::FieldReader<u16>;
///Field `LENG` writer - Non-volatile data segment length
pub type LENG_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16, crate::Safe>;
impl R {
    ///Bits 6:15 - Non-volatile data segment length
    #[inline(always)]
    pub fn leng(&self) -> LENG_R {
        LENG_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VDSL").field("leng", &self.leng()).finish()
    }
}
impl W {
    ///Bits 6:15 - Non-volatile data segment length
    #[inline(always)]
    pub fn leng(&mut self) -> LENG_W<'_, VDSLrs> {
        LENG_W::new(self, 6)
    }
}
/**Volatile data segment length

You can [`read`](crate::Reg::read) this register and get [`vdsl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdsl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#FIREWALL:VDSL)*/
pub struct VDSLrs;
impl crate::RegisterSpec for VDSLrs {
    type Ux = u32;
}
///`read()` method returns [`vdsl::R`](R) reader structure
impl crate::Readable for VDSLrs {}
///`write(|w| ..)` method takes [`vdsl::W`](W) writer structure
impl crate::Writable for VDSLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VDSL to value 0
impl crate::Resettable for VDSLrs {}
