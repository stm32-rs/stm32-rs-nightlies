///Register `INI6_FN_MOD` reader
pub type R = crate::R<INI6_FN_MODrs>;
///Register `INI6_FN_MOD` writer
pub type W = crate::W<INI6_FN_MODrs>;
///Field `READ_ISS_OVERRIDE` reader - Override ASIB read issuing capability
pub type READ_ISS_OVERRIDE_R = crate::BitReader;
///Field `READ_ISS_OVERRIDE` writer - Override ASIB read issuing capability
pub type READ_ISS_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRITE_ISS_OVERRIDE` reader - Override ASIB write issuing capability
pub type WRITE_ISS_OVERRIDE_R = crate::BitReader;
///Field `WRITE_ISS_OVERRIDE` writer - Override ASIB write issuing capability
pub type WRITE_ISS_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Override ASIB read issuing capability
    #[inline(always)]
    pub fn read_iss_override(&self) -> READ_ISS_OVERRIDE_R {
        READ_ISS_OVERRIDE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Override ASIB write issuing capability
    #[inline(always)]
    pub fn write_iss_override(&self) -> WRITE_ISS_OVERRIDE_R {
        WRITE_ISS_OVERRIDE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INI6_FN_MOD")
            .field("read_iss_override", &self.read_iss_override())
            .field("write_iss_override", &self.write_iss_override())
            .finish()
    }
}
impl W {
    ///Bit 0 - Override ASIB read issuing capability
    #[inline(always)]
    pub fn read_iss_override(&mut self) -> READ_ISS_OVERRIDE_W<'_, INI6_FN_MODrs> {
        READ_ISS_OVERRIDE_W::new(self, 0)
    }
    ///Bit 1 - Override ASIB write issuing capability
    #[inline(always)]
    pub fn write_iss_override(&mut self) -> WRITE_ISS_OVERRIDE_W<'_, INI6_FN_MODrs> {
        WRITE_ISS_OVERRIDE_W::new(self, 1)
    }
}
/**AXI interconnect - INI x issuing functionality modification register

You can [`read`](crate::Reg::read) this register and get [`ini6_fn_mod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ini6_fn_mod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#AXI:INI6_FN_MOD)*/
pub struct INI6_FN_MODrs;
impl crate::RegisterSpec for INI6_FN_MODrs {
    type Ux = u32;
}
///`read()` method returns [`ini6_fn_mod::R`](R) reader structure
impl crate::Readable for INI6_FN_MODrs {}
///`write(|w| ..)` method takes [`ini6_fn_mod::W`](W) writer structure
impl crate::Writable for INI6_FN_MODrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INI6_FN_MOD to value 0x04
impl crate::Resettable for INI6_FN_MODrs {
    const RESET_VALUE: u32 = 0x04;
}
