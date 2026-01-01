///Register `INI1_FN_MOD` reader
pub type R = crate::R<INI1_FN_MODrs>;
///Register `INI1_FN_MOD` writer
pub type W = crate::W<INI1_FN_MODrs>;
/**Override ASIB read issuing capability

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum READ_ISS_OVERRIDE {
    ///0: Normal ASIB read issuing capability
    Normal = 0,
    ///1: Force ASIB read issuing capability to 1
    Force1 = 1,
}
impl From<READ_ISS_OVERRIDE> for bool {
    #[inline(always)]
    fn from(variant: READ_ISS_OVERRIDE) -> Self {
        variant as u8 != 0
    }
}
///Field `READ_ISS_OVERRIDE` reader - Override ASIB read issuing capability
pub type READ_ISS_OVERRIDE_R = crate::BitReader<READ_ISS_OVERRIDE>;
impl READ_ISS_OVERRIDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> READ_ISS_OVERRIDE {
        match self.bits {
            false => READ_ISS_OVERRIDE::Normal,
            true => READ_ISS_OVERRIDE::Force1,
        }
    }
    ///Normal ASIB read issuing capability
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == READ_ISS_OVERRIDE::Normal
    }
    ///Force ASIB read issuing capability to 1
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == READ_ISS_OVERRIDE::Force1
    }
}
///Field `READ_ISS_OVERRIDE` writer - Override ASIB read issuing capability
pub type READ_ISS_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG, READ_ISS_OVERRIDE>;
impl<'a, REG> READ_ISS_OVERRIDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal ASIB read issuing capability
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(READ_ISS_OVERRIDE::Normal)
    }
    ///Force ASIB read issuing capability to 1
    #[inline(always)]
    pub fn force1(self) -> &'a mut crate::W<REG> {
        self.variant(READ_ISS_OVERRIDE::Force1)
    }
}
/**Override ASIB write issuing capability

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRITE_ISS_OVERRIDE {
    ///0: Normal ASIB write issuing capability
    Normal = 0,
    ///1: Force ASIB write issuing capability to 1
    Force1 = 1,
}
impl From<WRITE_ISS_OVERRIDE> for bool {
    #[inline(always)]
    fn from(variant: WRITE_ISS_OVERRIDE) -> Self {
        variant as u8 != 0
    }
}
///Field `WRITE_ISS_OVERRIDE` reader - Override ASIB write issuing capability
pub type WRITE_ISS_OVERRIDE_R = crate::BitReader<WRITE_ISS_OVERRIDE>;
impl WRITE_ISS_OVERRIDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WRITE_ISS_OVERRIDE {
        match self.bits {
            false => WRITE_ISS_OVERRIDE::Normal,
            true => WRITE_ISS_OVERRIDE::Force1,
        }
    }
    ///Normal ASIB write issuing capability
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == WRITE_ISS_OVERRIDE::Normal
    }
    ///Force ASIB write issuing capability to 1
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == WRITE_ISS_OVERRIDE::Force1
    }
}
///Field `WRITE_ISS_OVERRIDE` writer - Override ASIB write issuing capability
pub type WRITE_ISS_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG, WRITE_ISS_OVERRIDE>;
impl<'a, REG> WRITE_ISS_OVERRIDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal ASIB write issuing capability
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(WRITE_ISS_OVERRIDE::Normal)
    }
    ///Force ASIB write issuing capability to 1
    #[inline(always)]
    pub fn force1(self) -> &'a mut crate::W<REG> {
        self.variant(WRITE_ISS_OVERRIDE::Force1)
    }
}
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
        f.debug_struct("INI1_FN_MOD")
            .field("read_iss_override", &self.read_iss_override())
            .field("write_iss_override", &self.write_iss_override())
            .finish()
    }
}
impl W {
    ///Bit 0 - Override ASIB read issuing capability
    #[inline(always)]
    pub fn read_iss_override(&mut self) -> READ_ISS_OVERRIDE_W<'_, INI1_FN_MODrs> {
        READ_ISS_OVERRIDE_W::new(self, 0)
    }
    ///Bit 1 - Override ASIB write issuing capability
    #[inline(always)]
    pub fn write_iss_override(&mut self) -> WRITE_ISS_OVERRIDE_W<'_, INI1_FN_MODrs> {
        WRITE_ISS_OVERRIDE_W::new(self, 1)
    }
}
/**AXI interconnect - INI x issuing functionality modification register

You can [`read`](crate::Reg::read) this register and get [`ini1_fn_mod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ini1_fn_mod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#AXI:INI1_FN_MOD)*/
pub struct INI1_FN_MODrs;
impl crate::RegisterSpec for INI1_FN_MODrs {
    type Ux = u32;
}
///`read()` method returns [`ini1_fn_mod::R`](R) reader structure
impl crate::Readable for INI1_FN_MODrs {}
///`write(|w| ..)` method takes [`ini1_fn_mod::W`](W) writer structure
impl crate::Writable for INI1_FN_MODrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INI1_FN_MOD to value 0x04
impl crate::Resettable for INI1_FN_MODrs {
    const RESET_VALUE: u32 = 0x04;
}
