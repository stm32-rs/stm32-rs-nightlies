#[doc = "Register `VOSCR` reader"]
pub type R = crate::R<VOSCRrs>;
#[doc = "Register `VOSCR` writer"]
pub type W = crate::W<VOSCRrs>;
#[doc = "voltage scaling selection according to performance These bits control the V&lt;sub>CORE&lt;/sub> voltage level and allow to obtain the best trade-off between power consumption and performance: - In bypass mode, these bits must also be set according to the external provided core voltage level and related performance. - When increasing the performance, the voltage scaling must be changed before increasing the system frequency. - When decreasing performance, the system frequency must first be decreased before changing the voltage scaling.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VOS {
    #[doc = "0: Scale 3 (default)"]
    Vos3 = 0,
    #[doc = "1: Scale 1"]
    Vos1 = 1,
    #[doc = "2: Scale 2"]
    Vos2 = 2,
    #[doc = "3: Scale 0"]
    Vos0 = 3,
}
impl From<VOS> for u8 {
    #[inline(always)]
    fn from(variant: VOS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VOS {
    type Ux = u8;
}
#[doc = "Field `VOS` reader - voltage scaling selection according to performance These bits control the V&lt;sub>CORE&lt;/sub> voltage level and allow to obtain the best trade-off between power consumption and performance: - In bypass mode, these bits must also be set according to the external provided core voltage level and related performance. - When increasing the performance, the voltage scaling must be changed before increasing the system frequency. - When decreasing performance, the system frequency must first be decreased before changing the voltage scaling."]
pub type VOS_R = crate::FieldReader<VOS>;
impl VOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VOS {
        match self.bits {
            0 => VOS::Vos3,
            1 => VOS::Vos1,
            2 => VOS::Vos2,
            3 => VOS::Vos0,
            _ => unreachable!(),
        }
    }
    #[doc = "Scale 3 (default)"]
    #[inline(always)]
    pub fn is_vos3(&self) -> bool {
        *self == VOS::Vos3
    }
    #[doc = "Scale 1"]
    #[inline(always)]
    pub fn is_vos1(&self) -> bool {
        *self == VOS::Vos1
    }
    #[doc = "Scale 2"]
    #[inline(always)]
    pub fn is_vos2(&self) -> bool {
        *self == VOS::Vos2
    }
    #[doc = "Scale 0"]
    #[inline(always)]
    pub fn is_vos0(&self) -> bool {
        *self == VOS::Vos0
    }
}
#[doc = "Field `VOS` writer - voltage scaling selection according to performance These bits control the V&lt;sub>CORE&lt;/sub> voltage level and allow to obtain the best trade-off between power consumption and performance: - In bypass mode, these bits must also be set according to the external provided core voltage level and related performance. - When increasing the performance, the voltage scaling must be changed before increasing the system frequency. - When decreasing performance, the system frequency must first be decreased before changing the voltage scaling."]
pub type VOS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, VOS>;
impl<'a, REG> VOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Scale 3 (default)"]
    #[inline(always)]
    pub fn vos3(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::Vos3)
    }
    #[doc = "Scale 1"]
    #[inline(always)]
    pub fn vos1(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::Vos1)
    }
    #[doc = "Scale 2"]
    #[inline(always)]
    pub fn vos2(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::Vos2)
    }
    #[doc = "Scale 0"]
    #[inline(always)]
    pub fn vos0(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::Vos0)
    }
}
impl R {
    #[doc = "Bits 4:5 - voltage scaling selection according to performance These bits control the V&lt;sub>CORE&lt;/sub> voltage level and allow to obtain the best trade-off between power consumption and performance: - In bypass mode, these bits must also be set according to the external provided core voltage level and related performance. - When increasing the performance, the voltage scaling must be changed before increasing the system frequency. - When decreasing performance, the system frequency must first be decreased before changing the voltage scaling."]
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - voltage scaling selection according to performance These bits control the V&lt;sub>CORE&lt;/sub> voltage level and allow to obtain the best trade-off between power consumption and performance: - In bypass mode, these bits must also be set according to the external provided core voltage level and related performance. - When increasing the performance, the voltage scaling must be changed before increasing the system frequency. - When decreasing performance, the system frequency must first be decreased before changing the voltage scaling."]
    #[inline(always)]
    #[must_use]
    pub fn vos(&mut self) -> VOS_W<VOSCRrs> {
        VOS_W::new(self, 4)
    }
}
#[doc = "PWR voltage scaling control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`voscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`voscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VOSCRrs;
impl crate::RegisterSpec for VOSCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`voscr::R`](R) reader structure"]
impl crate::Readable for VOSCRrs {}
#[doc = "`write(|w| ..)` method takes [`voscr::W`](W) writer structure"]
impl crate::Writable for VOSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VOSCR to value 0"]
impl crate::Resettable for VOSCRrs {
    const RESET_VALUE: u32 = 0;
}
