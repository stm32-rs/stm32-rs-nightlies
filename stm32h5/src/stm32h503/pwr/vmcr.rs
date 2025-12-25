///Register `VMCR` reader
pub type R = crate::R<VMCRrs>;
///Register `VMCR` writer
pub type W = crate::W<VMCRrs>;
/**PVD enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDE {
    ///0: PVD Disabled
    Disabled = 0,
    ///1: PVD Enabled
    Enabled = 1,
}
impl From<PVDE> for bool {
    #[inline(always)]
    fn from(variant: PVDE) -> Self {
        variant as u8 != 0
    }
}
///Field `PVDE` reader - PVD enable
pub type PVDE_R = crate::BitReader<PVDE>;
impl PVDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PVDE {
        match self.bits {
            false => PVDE::Disabled,
            true => PVDE::Enabled,
        }
    }
    ///PVD Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PVDE::Disabled
    }
    ///PVD Enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PVDE::Enabled
    }
}
///Field `PVDE` writer - PVD enable
pub type PVDE_W<'a, REG> = crate::BitWriter<'a, REG, PVDE>;
impl<'a, REG> PVDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PVD Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVDE::Disabled)
    }
    ///PVD Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVDE::Enabled)
    }
}
/**programmable voltage detector (PVD) level selection These bits select the voltage threshold detected by the PVD.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLS {
    ///0: PVD level0 (VPVD0 around 1.95 V)
    PvdLevel0 = 0,
    ///1: PVD level1 (VPVD1 around 2.1 V)
    PvdLevel1 = 1,
    ///2: PVD level2 (VPVD2 around 2.25 V)
    PvdLevel2 = 2,
    ///3: PVD level3 (VPVD3 around 2.4 V)
    PvdLevel3 = 3,
    ///4: PVD level4 (VPVD4 around 2.55 V)
    PvdLevel4 = 4,
    ///5: PVD level5 (VPVD5 around 2.7 V)
    PvdLevel5 = 5,
    ///6: PVD level6 (VPVD6 around 2.85 V)
    PvdLevel6 = 6,
    ///7: PVD_IN pin
    PvdIn = 7,
}
impl From<PLS> for u8 {
    #[inline(always)]
    fn from(variant: PLS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLS {
    type Ux = u8;
}
impl crate::IsEnum for PLS {}
///Field `PLS` reader - programmable voltage detector (PVD) level selection These bits select the voltage threshold detected by the PVD.
pub type PLS_R = crate::FieldReader<PLS>;
impl PLS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLS {
        match self.bits {
            0 => PLS::PvdLevel0,
            1 => PLS::PvdLevel1,
            2 => PLS::PvdLevel2,
            3 => PLS::PvdLevel3,
            4 => PLS::PvdLevel4,
            5 => PLS::PvdLevel5,
            6 => PLS::PvdLevel6,
            7 => PLS::PvdIn,
            _ => unreachable!(),
        }
    }
    ///PVD level0 (VPVD0 around 1.95 V)
    #[inline(always)]
    pub fn is_pvd_level0(&self) -> bool {
        *self == PLS::PvdLevel0
    }
    ///PVD level1 (VPVD1 around 2.1 V)
    #[inline(always)]
    pub fn is_pvd_level1(&self) -> bool {
        *self == PLS::PvdLevel1
    }
    ///PVD level2 (VPVD2 around 2.25 V)
    #[inline(always)]
    pub fn is_pvd_level2(&self) -> bool {
        *self == PLS::PvdLevel2
    }
    ///PVD level3 (VPVD3 around 2.4 V)
    #[inline(always)]
    pub fn is_pvd_level3(&self) -> bool {
        *self == PLS::PvdLevel3
    }
    ///PVD level4 (VPVD4 around 2.55 V)
    #[inline(always)]
    pub fn is_pvd_level4(&self) -> bool {
        *self == PLS::PvdLevel4
    }
    ///PVD level5 (VPVD5 around 2.7 V)
    #[inline(always)]
    pub fn is_pvd_level5(&self) -> bool {
        *self == PLS::PvdLevel5
    }
    ///PVD level6 (VPVD6 around 2.85 V)
    #[inline(always)]
    pub fn is_pvd_level6(&self) -> bool {
        *self == PLS::PvdLevel6
    }
    ///PVD_IN pin
    #[inline(always)]
    pub fn is_pvd_in(&self) -> bool {
        *self == PLS::PvdIn
    }
}
///Field `PLS` writer - programmable voltage detector (PVD) level selection These bits select the voltage threshold detected by the PVD.
pub type PLS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PLS, crate::Safe>;
impl<'a, REG> PLS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PVD level0 (VPVD0 around 1.95 V)
    #[inline(always)]
    pub fn pvd_level0(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::PvdLevel0)
    }
    ///PVD level1 (VPVD1 around 2.1 V)
    #[inline(always)]
    pub fn pvd_level1(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::PvdLevel1)
    }
    ///PVD level2 (VPVD2 around 2.25 V)
    #[inline(always)]
    pub fn pvd_level2(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::PvdLevel2)
    }
    ///PVD level3 (VPVD3 around 2.4 V)
    #[inline(always)]
    pub fn pvd_level3(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::PvdLevel3)
    }
    ///PVD level4 (VPVD4 around 2.55 V)
    #[inline(always)]
    pub fn pvd_level4(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::PvdLevel4)
    }
    ///PVD level5 (VPVD5 around 2.7 V)
    #[inline(always)]
    pub fn pvd_level5(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::PvdLevel5)
    }
    ///PVD level6 (VPVD6 around 2.85 V)
    #[inline(always)]
    pub fn pvd_level6(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::PvdLevel6)
    }
    ///PVD_IN pin
    #[inline(always)]
    pub fn pvd_in(self) -> &'a mut crate::W<REG> {
        self.variant(PLS::PvdIn)
    }
}
/**peripheral voltage monitor on V DDA enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVDEN {
    ///0: Peripheral voltage monitor on VDDA disabled
    Disabled = 0,
    ///1: Peripheral voltage monitor on VDDA enabled
    Enabled = 1,
}
impl From<AVDEN> for bool {
    #[inline(always)]
    fn from(variant: AVDEN) -> Self {
        variant as u8 != 0
    }
}
///Field `AVDEN` reader - peripheral voltage monitor on V DDA enable
pub type AVDEN_R = crate::BitReader<AVDEN>;
impl AVDEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AVDEN {
        match self.bits {
            false => AVDEN::Disabled,
            true => AVDEN::Enabled,
        }
    }
    ///Peripheral voltage monitor on VDDA disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AVDEN::Disabled
    }
    ///Peripheral voltage monitor on VDDA enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AVDEN::Enabled
    }
}
///Field `AVDEN` writer - peripheral voltage monitor on V DDA enable
pub type AVDEN_W<'a, REG> = crate::BitWriter<'a, REG, AVDEN>;
impl<'a, REG> AVDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral voltage monitor on VDDA disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AVDEN::Disabled)
    }
    ///Peripheral voltage monitor on VDDA enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AVDEN::Enabled)
    }
}
/**analog voltage detector (AVD) level selection These bits select the voltage threshold detected by the AVD.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ALS {
    ///0: AVD level0 (VAVD0 around 1.7 V)
    AvdLevel0 = 0,
    ///1: AVD level1 (VAVD1 around 2.1 V)
    AvdLevel1 = 1,
    ///2: AVD level2 (VAVD2 around 2.5 V)
    AvdLevel2 = 2,
    ///3: AVD level3 (VAVD3 around 2.8 V)
    AvdLevel3 = 3,
}
impl From<ALS> for u8 {
    #[inline(always)]
    fn from(variant: ALS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ALS {
    type Ux = u8;
}
impl crate::IsEnum for ALS {}
///Field `ALS` reader - analog voltage detector (AVD) level selection These bits select the voltage threshold detected by the AVD.
pub type ALS_R = crate::FieldReader<ALS>;
impl ALS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ALS {
        match self.bits {
            0 => ALS::AvdLevel0,
            1 => ALS::AvdLevel1,
            2 => ALS::AvdLevel2,
            3 => ALS::AvdLevel3,
            _ => unreachable!(),
        }
    }
    ///AVD level0 (VAVD0 around 1.7 V)
    #[inline(always)]
    pub fn is_avd_level0(&self) -> bool {
        *self == ALS::AvdLevel0
    }
    ///AVD level1 (VAVD1 around 2.1 V)
    #[inline(always)]
    pub fn is_avd_level1(&self) -> bool {
        *self == ALS::AvdLevel1
    }
    ///AVD level2 (VAVD2 around 2.5 V)
    #[inline(always)]
    pub fn is_avd_level2(&self) -> bool {
        *self == ALS::AvdLevel2
    }
    ///AVD level3 (VAVD3 around 2.8 V)
    #[inline(always)]
    pub fn is_avd_level3(&self) -> bool {
        *self == ALS::AvdLevel3
    }
}
///Field `ALS` writer - analog voltage detector (AVD) level selection These bits select the voltage threshold detected by the AVD.
pub type ALS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ALS, crate::Safe>;
impl<'a, REG> ALS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///AVD level0 (VAVD0 around 1.7 V)
    #[inline(always)]
    pub fn avd_level0(self) -> &'a mut crate::W<REG> {
        self.variant(ALS::AvdLevel0)
    }
    ///AVD level1 (VAVD1 around 2.1 V)
    #[inline(always)]
    pub fn avd_level1(self) -> &'a mut crate::W<REG> {
        self.variant(ALS::AvdLevel1)
    }
    ///AVD level2 (VAVD2 around 2.5 V)
    #[inline(always)]
    pub fn avd_level2(self) -> &'a mut crate::W<REG> {
        self.variant(ALS::AvdLevel2)
    }
    ///AVD level3 (VAVD3 around 2.8 V)
    #[inline(always)]
    pub fn avd_level3(self) -> &'a mut crate::W<REG> {
        self.variant(ALS::AvdLevel3)
    }
}
impl R {
    ///Bit 0 - PVD enable
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - programmable voltage detector (PVD) level selection These bits select the voltage threshold detected by the PVD.
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bit 8 - peripheral voltage monitor on V DDA enable
    #[inline(always)]
    pub fn avden(&self) -> AVDEN_R {
        AVDEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - analog voltage detector (AVD) level selection These bits select the voltage threshold detected by the AVD.
    #[inline(always)]
    pub fn als(&self) -> ALS_R {
        ALS_R::new(((self.bits >> 9) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VMCR")
            .field("pvde", &self.pvde())
            .field("pls", &self.pls())
            .field("avden", &self.avden())
            .field("als", &self.als())
            .finish()
    }
}
impl W {
    ///Bit 0 - PVD enable
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W<'_, VMCRrs> {
        PVDE_W::new(self, 0)
    }
    ///Bits 1:3 - programmable voltage detector (PVD) level selection These bits select the voltage threshold detected by the PVD.
    #[inline(always)]
    pub fn pls(&mut self) -> PLS_W<'_, VMCRrs> {
        PLS_W::new(self, 1)
    }
    ///Bit 8 - peripheral voltage monitor on V DDA enable
    #[inline(always)]
    pub fn avden(&mut self) -> AVDEN_W<'_, VMCRrs> {
        AVDEN_W::new(self, 8)
    }
    ///Bits 9:10 - analog voltage detector (AVD) level selection These bits select the voltage threshold detected by the AVD.
    #[inline(always)]
    pub fn als(&mut self) -> ALS_W<'_, VMCRrs> {
        ALS_W::new(self, 9)
    }
}
/**PWR voltage monitor control register

You can [`read`](crate::Reg::read) this register and get [`vmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#PWR:VMCR)*/
pub struct VMCRrs;
impl crate::RegisterSpec for VMCRrs {
    type Ux = u32;
}
///`read()` method returns [`vmcr::R`](R) reader structure
impl crate::Readable for VMCRrs {}
///`write(|w| ..)` method takes [`vmcr::W`](W) writer structure
impl crate::Writable for VMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets VMCR to value 0
impl crate::Resettable for VMCRrs {}
