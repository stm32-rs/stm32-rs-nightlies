#[doc = "Register `I3C_TIMINGR2` reader"]
pub type R = crate::R<I3C_TIMINGR2rs>;
#[doc = "Register `I3C_TIMINGR2` writer"]
pub type W = crate::W<I3C_TIMINGR2rs>;
#[doc = "Field `STALLT` reader - Controller clock stall on T-bit phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to prepare data to be sent."]
pub type STALLT_R = crate::BitReader;
#[doc = "Field `STALLT` writer - Controller clock stall on T-bit phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to prepare data to be sent."]
pub type STALLT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLD` reader - controller clock stall on PAR phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to read received data."]
pub type STALLD_R = crate::BitReader;
#[doc = "Field `STALLD` writer - controller clock stall on PAR phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to read received data."]
pub type STALLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLC` reader - controller clock stall on PAR phase of CCC enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase of common command code (before 9th bit). This allows the target to decode the command."]
pub type STALLC_R = crate::BitReader;
#[doc = "Field `STALLC` writer - controller clock stall on PAR phase of CCC enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase of common command code (before 9th bit). This allows the target to decode the command."]
pub type STALLC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLA` reader - controller clock stall enable on ACK phase The SCL is stalled (during tSCLL_STALLas defined by STALL) in the address ACK/NACK phase (before 9th bit). This allows the target to prepare data or the controller to respond to target interrupt."]
pub type STALLA_R = crate::BitReader;
#[doc = "Field `STALLA` writer - controller clock stall enable on ACK phase The SCL is stalled (during tSCLL_STALLas defined by STALL) in the address ACK/NACK phase (before 9th bit). This allows the target to prepare data or the controller to respond to target interrupt."]
pub type STALLA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - controller clock stall time, in number of kernel clock cycles tSCLL_STALL = STALL x tI3CCLK"]
pub type STALL_R = crate::FieldReader;
#[doc = "Field `STALL` writer - controller clock stall time, in number of kernel clock cycles tSCLL_STALL = STALL x tI3CCLK"]
pub type STALL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Controller clock stall on T-bit phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to prepare data to be sent."]
    #[inline(always)]
    pub fn stallt(&self) -> STALLT_R {
        STALLT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - controller clock stall on PAR phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to read received data."]
    #[inline(always)]
    pub fn stalld(&self) -> STALLD_R {
        STALLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - controller clock stall on PAR phase of CCC enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase of common command code (before 9th bit). This allows the target to decode the command."]
    #[inline(always)]
    pub fn stallc(&self) -> STALLC_R {
        STALLC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - controller clock stall enable on ACK phase The SCL is stalled (during tSCLL_STALLas defined by STALL) in the address ACK/NACK phase (before 9th bit). This allows the target to prepare data or the controller to respond to target interrupt."]
    #[inline(always)]
    pub fn stalla(&self) -> STALLA_R {
        STALLA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:15 - controller clock stall time, in number of kernel clock cycles tSCLL_STALL = STALL x tI3CCLK"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Controller clock stall on T-bit phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to prepare data to be sent."]
    #[inline(always)]
    #[must_use]
    pub fn stallt(&mut self) -> STALLT_W<I3C_TIMINGR2rs> {
        STALLT_W::new(self, 0)
    }
    #[doc = "Bit 1 - controller clock stall on PAR phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to read received data."]
    #[inline(always)]
    #[must_use]
    pub fn stalld(&mut self) -> STALLD_W<I3C_TIMINGR2rs> {
        STALLD_W::new(self, 1)
    }
    #[doc = "Bit 2 - controller clock stall on PAR phase of CCC enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase of common command code (before 9th bit). This allows the target to decode the command."]
    #[inline(always)]
    #[must_use]
    pub fn stallc(&mut self) -> STALLC_W<I3C_TIMINGR2rs> {
        STALLC_W::new(self, 2)
    }
    #[doc = "Bit 3 - controller clock stall enable on ACK phase The SCL is stalled (during tSCLL_STALLas defined by STALL) in the address ACK/NACK phase (before 9th bit). This allows the target to prepare data or the controller to respond to target interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn stalla(&mut self) -> STALLA_W<I3C_TIMINGR2rs> {
        STALLA_W::new(self, 3)
    }
    #[doc = "Bits 8:15 - controller clock stall time, in number of kernel clock cycles tSCLL_STALL = STALL x tI3CCLK"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<I3C_TIMINGR2rs> {
        STALL_W::new(self, 8)
    }
}
#[doc = "I3C timing register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i3c_timingr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i3c_timingr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3C_TIMINGR2rs;
impl crate::RegisterSpec for I3C_TIMINGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3c_timingr2::R`](R) reader structure"]
impl crate::Readable for I3C_TIMINGR2rs {}
#[doc = "`write(|w| ..)` method takes [`i3c_timingr2::W`](W) writer structure"]
impl crate::Writable for I3C_TIMINGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I3C_TIMINGR2 to value 0"]
impl crate::Resettable for I3C_TIMINGR2rs {
    const RESET_VALUE: u32 = 0;
}
