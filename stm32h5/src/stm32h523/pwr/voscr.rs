///Register `VOSCR` reader
pub type R = crate::R<VOSCRrs>;
///Register `VOSCR` writer
pub type W = crate::W<VOSCRrs>;
/**voltage scaling selection according to performance

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VOS {
    ///0: Scale 3 (default)
    Vos3 = 0,
    ///1: Scale 1
    Vos1 = 1,
    ///2: Scale 2
    Vos2 = 2,
    ///3: Scale 0
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
impl crate::IsEnum for VOS {}
///Field `VOS` reader - voltage scaling selection according to performance
pub type VOS_R = crate::FieldReader<VOS>;
impl VOS_R {
    ///Get enumerated values variant
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
    ///Scale 3 (default)
    #[inline(always)]
    pub fn is_vos3(&self) -> bool {
        *self == VOS::Vos3
    }
    ///Scale 1
    #[inline(always)]
    pub fn is_vos1(&self) -> bool {
        *self == VOS::Vos1
    }
    ///Scale 2
    #[inline(always)]
    pub fn is_vos2(&self) -> bool {
        *self == VOS::Vos2
    }
    ///Scale 0
    #[inline(always)]
    pub fn is_vos0(&self) -> bool {
        *self == VOS::Vos0
    }
}
///Field `VOS` writer - voltage scaling selection according to performance
pub type VOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, VOS, crate::Safe>;
impl<'a, REG> VOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Scale 3 (default)
    #[inline(always)]
    pub fn vos3(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::Vos3)
    }
    ///Scale 1
    #[inline(always)]
    pub fn vos1(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::Vos1)
    }
    ///Scale 2
    #[inline(always)]
    pub fn vos2(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::Vos2)
    }
    ///Scale 0
    #[inline(always)]
    pub fn vos0(self) -> &'a mut crate::W<REG> {
        self.variant(VOS::Vos0)
    }
}
impl R {
    ///Bits 4:5 - voltage scaling selection according to performance
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VOSCR").field("vos", &self.vos()).finish()
    }
}
impl W {
    ///Bits 4:5 - voltage scaling selection according to performance
    #[inline(always)]
    pub fn vos(&mut self) -> VOS_W<'_, VOSCRrs> {
        VOS_W::new(self, 4)
    }
}
/**PWR voltage scaling control register

You can [`read`](crate::Reg::read) this register and get [`voscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`voscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#PWR:VOSCR)*/
pub struct VOSCRrs;
impl crate::RegisterSpec for VOSCRrs {
    type Ux = u32;
}
///`read()` method returns [`voscr::R`](R) reader structure
impl crate::Readable for VOSCRrs {}
///`write(|w| ..)` method takes [`voscr::W`](W) writer structure
impl crate::Writable for VOSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VOSCR to value 0
impl crate::Resettable for VOSCRrs {}
