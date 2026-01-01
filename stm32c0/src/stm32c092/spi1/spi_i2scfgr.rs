///Register `SPI_I2SCFGR` reader
pub type R = crate::R<SPI_I2SCFGRrs>;
///Register `SPI_I2SCFGR` writer
pub type W = crate::W<SPI_I2SCFGRrs>;
/**Channel length (number of bits per audio channel) The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled. Note: It is not used in SPI mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHLEN {
    ///0: 16-bit wide
    B0x0 = 0,
    ///1: 32-bit wide
    B0x1 = 1,
}
impl From<CHLEN> for bool {
    #[inline(always)]
    fn from(variant: CHLEN) -> Self {
        variant as u8 != 0
    }
}
///Field `CHLEN` reader - Channel length (number of bits per audio channel) The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled. Note: It is not used in SPI mode.
pub type CHLEN_R = crate::BitReader<CHLEN>;
impl CHLEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CHLEN {
        match self.bits {
            false => CHLEN::B0x0,
            true => CHLEN::B0x1,
        }
    }
    ///16-bit wide
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHLEN::B0x0
    }
    ///32-bit wide
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHLEN::B0x1
    }
}
///Field `CHLEN` writer - Channel length (number of bits per audio channel) The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled. Note: It is not used in SPI mode.
pub type CHLEN_W<'a, REG> = crate::BitWriter<'a, REG, CHLEN>;
impl<'a, REG> CHLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///16-bit wide
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CHLEN::B0x0)
    }
    ///32-bit wide
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CHLEN::B0x1)
    }
}
/**Data length to be transferred Note: For correct operation, these bits should be configured when the I2S is disabled. Note: They are not used in SPI mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATLEN {
    ///0: 16-bit data length
    B0x0 = 0,
    ///1: 24-bit data length
    B0x1 = 1,
    ///2: 32-bit data length
    B0x2 = 2,
    ///3: Not allowed
    B0x3 = 3,
}
impl From<DATLEN> for u8 {
    #[inline(always)]
    fn from(variant: DATLEN) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DATLEN {
    type Ux = u8;
}
impl crate::IsEnum for DATLEN {}
///Field `DATLEN` reader - Data length to be transferred Note: For correct operation, these bits should be configured when the I2S is disabled. Note: They are not used in SPI mode.
pub type DATLEN_R = crate::FieldReader<DATLEN>;
impl DATLEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DATLEN {
        match self.bits {
            0 => DATLEN::B0x0,
            1 => DATLEN::B0x1,
            2 => DATLEN::B0x2,
            3 => DATLEN::B0x3,
            _ => unreachable!(),
        }
    }
    ///16-bit data length
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DATLEN::B0x0
    }
    ///24-bit data length
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DATLEN::B0x1
    }
    ///32-bit data length
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == DATLEN::B0x2
    }
    ///Not allowed
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == DATLEN::B0x3
    }
}
///Field `DATLEN` writer - Data length to be transferred Note: For correct operation, these bits should be configured when the I2S is disabled. Note: They are not used in SPI mode.
pub type DATLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DATLEN, crate::Safe>;
impl<'a, REG> DATLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///16-bit data length
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DATLEN::B0x0)
    }
    ///24-bit data length
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DATLEN::B0x1)
    }
    ///32-bit data length
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(DATLEN::B0x2)
    }
    ///Not allowed
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(DATLEN::B0x3)
    }
}
/**Inactive state clock polarity Note: For correct operation, this bit should be configured when the I2S is disabled. Note: It is not used in SPI mode. Note: The bit CKPOL does not affect the CK edge sensitivity used to receive or transmit the SD and WS signals.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKPOL {
    ///0: I2S clock inactive state is low level
    B0x0 = 0,
    ///1: I2S clock inactive state is high level
    B0x1 = 1,
}
impl From<CKPOL> for bool {
    #[inline(always)]
    fn from(variant: CKPOL) -> Self {
        variant as u8 != 0
    }
}
///Field `CKPOL` reader - Inactive state clock polarity Note: For correct operation, this bit should be configured when the I2S is disabled. Note: It is not used in SPI mode. Note: The bit CKPOL does not affect the CK edge sensitivity used to receive or transmit the SD and WS signals.
pub type CKPOL_R = crate::BitReader<CKPOL>;
impl CKPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKPOL {
        match self.bits {
            false => CKPOL::B0x0,
            true => CKPOL::B0x1,
        }
    }
    ///I2S clock inactive state is low level
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CKPOL::B0x0
    }
    ///I2S clock inactive state is high level
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CKPOL::B0x1
    }
}
///Field `CKPOL` writer - Inactive state clock polarity Note: For correct operation, this bit should be configured when the I2S is disabled. Note: It is not used in SPI mode. Note: The bit CKPOL does not affect the CK edge sensitivity used to receive or transmit the SD and WS signals.
pub type CKPOL_W<'a, REG> = crate::BitWriter<'a, REG, CKPOL>;
impl<'a, REG> CKPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///I2S clock inactive state is low level
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL::B0x0)
    }
    ///I2S clock inactive state is high level
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL::B0x1)
    }
}
/**I2S standard selection For more details on I<sup>2</sup>S standards, refer to Section 27.7.2 on page 805 Note: For correct operation, these bits should be configured when the I2S is disabled. Note: They are not used in SPI mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2SSTD {
    ///0: I<sup>2</sup>S Philips standard
    B0x0 = 0,
    ///1: MSB justified standard (left justified)
    B0x1 = 1,
    ///2: LSB justified standard (right justified)
    B0x2 = 2,
    ///3: PCM standard
    B0x3 = 3,
}
impl From<I2SSTD> for u8 {
    #[inline(always)]
    fn from(variant: I2SSTD) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2SSTD {
    type Ux = u8;
}
impl crate::IsEnum for I2SSTD {}
///Field `I2SSTD` reader - I2S standard selection For more details on I<sup>2</sup>S standards, refer to Section 27.7.2 on page 805 Note: For correct operation, these bits should be configured when the I2S is disabled. Note: They are not used in SPI mode.
pub type I2SSTD_R = crate::FieldReader<I2SSTD>;
impl I2SSTD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2SSTD {
        match self.bits {
            0 => I2SSTD::B0x0,
            1 => I2SSTD::B0x1,
            2 => I2SSTD::B0x2,
            3 => I2SSTD::B0x3,
            _ => unreachable!(),
        }
    }
    ///I<sup>2</sup>S Philips standard
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2SSTD::B0x0
    }
    ///MSB justified standard (left justified)
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2SSTD::B0x1
    }
    ///LSB justified standard (right justified)
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == I2SSTD::B0x2
    }
    ///PCM standard
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == I2SSTD::B0x3
    }
}
///Field `I2SSTD` writer - I2S standard selection For more details on I<sup>2</sup>S standards, refer to Section 27.7.2 on page 805 Note: For correct operation, these bits should be configured when the I2S is disabled. Note: They are not used in SPI mode.
pub type I2SSTD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I2SSTD, crate::Safe>;
impl<'a, REG> I2SSTD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///I<sup>2</sup>S Philips standard
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD::B0x0)
    }
    ///MSB justified standard (left justified)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD::B0x1)
    }
    ///LSB justified standard (right justified)
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD::B0x2)
    }
    ///PCM standard
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(I2SSTD::B0x3)
    }
}
/**PCM frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). Note: It is not used in SPI mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCMSYNC {
    ///0: Short frame synchronization
    B0x0 = 0,
    ///1: Long frame synchronization
    B0x1 = 1,
}
impl From<PCMSYNC> for bool {
    #[inline(always)]
    fn from(variant: PCMSYNC) -> Self {
        variant as u8 != 0
    }
}
///Field `PCMSYNC` reader - PCM frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). Note: It is not used in SPI mode.
pub type PCMSYNC_R = crate::BitReader<PCMSYNC>;
impl PCMSYNC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PCMSYNC {
        match self.bits {
            false => PCMSYNC::B0x0,
            true => PCMSYNC::B0x1,
        }
    }
    ///Short frame synchronization
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PCMSYNC::B0x0
    }
    ///Long frame synchronization
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PCMSYNC::B0x1
    }
}
///Field `PCMSYNC` writer - PCM frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). Note: It is not used in SPI mode.
pub type PCMSYNC_W<'a, REG> = crate::BitWriter<'a, REG, PCMSYNC>;
impl<'a, REG> PCMSYNC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Short frame synchronization
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PCMSYNC::B0x0)
    }
    ///Long frame synchronization
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PCMSYNC::B0x1)
    }
}
/**I2S configuration mode Note: These bits should be configured when the I2S is disabled. Note: They are not used in SPI mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2SCFG {
    ///0: Slave - transmit
    B0x0 = 0,
    ///1: Slave - receive
    B0x1 = 1,
    ///2: Master - transmit
    B0x2 = 2,
    ///3: Master - receive
    B0x3 = 3,
}
impl From<I2SCFG> for u8 {
    #[inline(always)]
    fn from(variant: I2SCFG) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2SCFG {
    type Ux = u8;
}
impl crate::IsEnum for I2SCFG {}
///Field `I2SCFG` reader - I2S configuration mode Note: These bits should be configured when the I2S is disabled. Note: They are not used in SPI mode.
pub type I2SCFG_R = crate::FieldReader<I2SCFG>;
impl I2SCFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2SCFG {
        match self.bits {
            0 => I2SCFG::B0x0,
            1 => I2SCFG::B0x1,
            2 => I2SCFG::B0x2,
            3 => I2SCFG::B0x3,
            _ => unreachable!(),
        }
    }
    ///Slave - transmit
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2SCFG::B0x0
    }
    ///Slave - receive
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2SCFG::B0x1
    }
    ///Master - transmit
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == I2SCFG::B0x2
    }
    ///Master - receive
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == I2SCFG::B0x3
    }
}
///Field `I2SCFG` writer - I2S configuration mode Note: These bits should be configured when the I2S is disabled. Note: They are not used in SPI mode.
pub type I2SCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 2, I2SCFG, crate::Safe>;
impl<'a, REG> I2SCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Slave - transmit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG::B0x0)
    }
    ///Slave - receive
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG::B0x1)
    }
    ///Master - transmit
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG::B0x2)
    }
    ///Master - receive
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(I2SCFG::B0x3)
    }
}
/**I2S enable Note: This bit is not used in SPI mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2SE {
    ///0: I2S peripheral is disabled
    B0x0 = 0,
    ///1: I2S peripheral is enabled
    B0x1 = 1,
}
impl From<I2SE> for bool {
    #[inline(always)]
    fn from(variant: I2SE) -> Self {
        variant as u8 != 0
    }
}
///Field `I2SE` reader - I2S enable Note: This bit is not used in SPI mode.
pub type I2SE_R = crate::BitReader<I2SE>;
impl I2SE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2SE {
        match self.bits {
            false => I2SE::B0x0,
            true => I2SE::B0x1,
        }
    }
    ///I2S peripheral is disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2SE::B0x0
    }
    ///I2S peripheral is enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2SE::B0x1
    }
}
///Field `I2SE` writer - I2S enable Note: This bit is not used in SPI mode.
pub type I2SE_W<'a, REG> = crate::BitWriter<'a, REG, I2SE>;
impl<'a, REG> I2SE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///I2S peripheral is disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2SE::B0x0)
    }
    ///I2S peripheral is enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2SE::B0x1)
    }
}
/**I2S mode selection Note: This bit should be configured when the SPI is disabled.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2SMOD {
    ///0: SPI mode is selected
    B0x0 = 0,
    ///1: I2S mode is selected
    B0x1 = 1,
}
impl From<I2SMOD> for bool {
    #[inline(always)]
    fn from(variant: I2SMOD) -> Self {
        variant as u8 != 0
    }
}
///Field `I2SMOD` reader - I2S mode selection Note: This bit should be configured when the SPI is disabled.
pub type I2SMOD_R = crate::BitReader<I2SMOD>;
impl I2SMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2SMOD {
        match self.bits {
            false => I2SMOD::B0x0,
            true => I2SMOD::B0x1,
        }
    }
    ///SPI mode is selected
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2SMOD::B0x0
    }
    ///I2S mode is selected
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2SMOD::B0x1
    }
}
///Field `I2SMOD` writer - I2S mode selection Note: This bit should be configured when the SPI is disabled.
pub type I2SMOD_W<'a, REG> = crate::BitWriter<'a, REG, I2SMOD>;
impl<'a, REG> I2SMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SPI mode is selected
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2SMOD::B0x0)
    }
    ///I2S mode is selected
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2SMOD::B0x1)
    }
}
/**Asynchronous start enable. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and an appropriate transition is detected on the WS signal. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and the appropriate level is detected on the WS signal. Note: The appropriate transition is a falling edge on WS signal when I<sup>2</sup>S Philips Standard is used, or a rising edge for other standards. Note: The appropriate level is a low level on WS signal when I<sup>2</sup>S Philips Standard is used, or a high level for other standards. Note: Please refer to Section 27.7.3: Start-up description for additional information.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASTRTEN {
    ///0: The Asynchronous start is disabled.
    B0x0 = 0,
    ///1: The Asynchronous start is enabled.
    B0x1 = 1,
}
impl From<ASTRTEN> for bool {
    #[inline(always)]
    fn from(variant: ASTRTEN) -> Self {
        variant as u8 != 0
    }
}
///Field `ASTRTEN` reader - Asynchronous start enable. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and an appropriate transition is detected on the WS signal. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and the appropriate level is detected on the WS signal. Note: The appropriate transition is a falling edge on WS signal when I<sup>2</sup>S Philips Standard is used, or a rising edge for other standards. Note: The appropriate level is a low level on WS signal when I<sup>2</sup>S Philips Standard is used, or a high level for other standards. Note: Please refer to Section 27.7.3: Start-up description for additional information.
pub type ASTRTEN_R = crate::BitReader<ASTRTEN>;
impl ASTRTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ASTRTEN {
        match self.bits {
            false => ASTRTEN::B0x0,
            true => ASTRTEN::B0x1,
        }
    }
    ///The Asynchronous start is disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ASTRTEN::B0x0
    }
    ///The Asynchronous start is enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ASTRTEN::B0x1
    }
}
///Field `ASTRTEN` writer - Asynchronous start enable. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and an appropriate transition is detected on the WS signal. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and the appropriate level is detected on the WS signal. Note: The appropriate transition is a falling edge on WS signal when I<sup>2</sup>S Philips Standard is used, or a rising edge for other standards. Note: The appropriate level is a low level on WS signal when I<sup>2</sup>S Philips Standard is used, or a high level for other standards. Note: Please refer to Section 27.7.3: Start-up description for additional information.
pub type ASTRTEN_W<'a, REG> = crate::BitWriter<'a, REG, ASTRTEN>;
impl<'a, REG> ASTRTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The Asynchronous start is disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ASTRTEN::B0x0)
    }
    ///The Asynchronous start is enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ASTRTEN::B0x1)
    }
}
impl R {
    ///Bit 0 - Channel length (number of bits per audio channel) The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled. Note: It is not used in SPI mode.
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Data length to be transferred Note: For correct operation, these bits should be configured when the I2S is disabled. Note: They are not used in SPI mode.
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - Inactive state clock polarity Note: For correct operation, this bit should be configured when the I2S is disabled. Note: It is not used in SPI mode. Note: The bit CKPOL does not affect the CK edge sensitivity used to receive or transmit the SD and WS signals.
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - I2S standard selection For more details on I<sup>2</sup>S standards, refer to Section 27.7.2 on page 805 Note: For correct operation, these bits should be configured when the I2S is disabled. Note: They are not used in SPI mode.
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 7 - PCM frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). Note: It is not used in SPI mode.
    #[inline(always)]
    pub fn pcmsync(&self) -> PCMSYNC_R {
        PCMSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - I2S configuration mode Note: These bits should be configured when the I2S is disabled. Note: They are not used in SPI mode.
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - I2S enable Note: This bit is not used in SPI mode.
    #[inline(always)]
    pub fn i2se(&self) -> I2SE_R {
        I2SE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - I2S mode selection Note: This bit should be configured when the SPI is disabled.
    #[inline(always)]
    pub fn i2smod(&self) -> I2SMOD_R {
        I2SMOD_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Asynchronous start enable. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and an appropriate transition is detected on the WS signal. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and the appropriate level is detected on the WS signal. Note: The appropriate transition is a falling edge on WS signal when I<sup>2</sup>S Philips Standard is used, or a rising edge for other standards. Note: The appropriate level is a low level on WS signal when I<sup>2</sup>S Philips Standard is used, or a high level for other standards. Note: Please refer to Section 27.7.3: Start-up description for additional information.
    #[inline(always)]
    pub fn astrten(&self) -> ASTRTEN_R {
        ASTRTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_I2SCFGR")
            .field("chlen", &self.chlen())
            .field("datlen", &self.datlen())
            .field("ckpol", &self.ckpol())
            .field("i2sstd", &self.i2sstd())
            .field("pcmsync", &self.pcmsync())
            .field("i2scfg", &self.i2scfg())
            .field("i2se", &self.i2se())
            .field("i2smod", &self.i2smod())
            .field("astrten", &self.astrten())
            .finish()
    }
}
impl W {
    ///Bit 0 - Channel length (number of bits per audio channel) The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled. Note: It is not used in SPI mode.
    #[inline(always)]
    pub fn chlen(&mut self) -> CHLEN_W<'_, SPI_I2SCFGRrs> {
        CHLEN_W::new(self, 0)
    }
    ///Bits 1:2 - Data length to be transferred Note: For correct operation, these bits should be configured when the I2S is disabled. Note: They are not used in SPI mode.
    #[inline(always)]
    pub fn datlen(&mut self) -> DATLEN_W<'_, SPI_I2SCFGRrs> {
        DATLEN_W::new(self, 1)
    }
    ///Bit 3 - Inactive state clock polarity Note: For correct operation, this bit should be configured when the I2S is disabled. Note: It is not used in SPI mode. Note: The bit CKPOL does not affect the CK edge sensitivity used to receive or transmit the SD and WS signals.
    #[inline(always)]
    pub fn ckpol(&mut self) -> CKPOL_W<'_, SPI_I2SCFGRrs> {
        CKPOL_W::new(self, 3)
    }
    ///Bits 4:5 - I2S standard selection For more details on I<sup>2</sup>S standards, refer to Section 27.7.2 on page 805 Note: For correct operation, these bits should be configured when the I2S is disabled. Note: They are not used in SPI mode.
    #[inline(always)]
    pub fn i2sstd(&mut self) -> I2SSTD_W<'_, SPI_I2SCFGRrs> {
        I2SSTD_W::new(self, 4)
    }
    ///Bit 7 - PCM frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). Note: It is not used in SPI mode.
    #[inline(always)]
    pub fn pcmsync(&mut self) -> PCMSYNC_W<'_, SPI_I2SCFGRrs> {
        PCMSYNC_W::new(self, 7)
    }
    ///Bits 8:9 - I2S configuration mode Note: These bits should be configured when the I2S is disabled. Note: They are not used in SPI mode.
    #[inline(always)]
    pub fn i2scfg(&mut self) -> I2SCFG_W<'_, SPI_I2SCFGRrs> {
        I2SCFG_W::new(self, 8)
    }
    ///Bit 10 - I2S enable Note: This bit is not used in SPI mode.
    #[inline(always)]
    pub fn i2se(&mut self) -> I2SE_W<'_, SPI_I2SCFGRrs> {
        I2SE_W::new(self, 10)
    }
    ///Bit 11 - I2S mode selection Note: This bit should be configured when the SPI is disabled.
    #[inline(always)]
    pub fn i2smod(&mut self) -> I2SMOD_W<'_, SPI_I2SCFGRrs> {
        I2SMOD_W::new(self, 11)
    }
    ///Bit 12 - Asynchronous start enable. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and an appropriate transition is detected on the WS signal. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and the appropriate level is detected on the WS signal. Note: The appropriate transition is a falling edge on WS signal when I<sup>2</sup>S Philips Standard is used, or a rising edge for other standards. Note: The appropriate level is a low level on WS signal when I<sup>2</sup>S Philips Standard is used, or a high level for other standards. Note: Please refer to Section 27.7.3: Start-up description for additional information.
    #[inline(always)]
    pub fn astrten(&mut self) -> ASTRTEN_W<'_, SPI_I2SCFGRrs> {
        ASTRTEN_W::new(self, 12)
    }
}
/**SPIx_I2S configuration register

You can [`read`](crate::Reg::read) this register and get [`spi_i2scfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_i2scfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#SPI1:SPI_I2SCFGR)*/
pub struct SPI_I2SCFGRrs;
impl crate::RegisterSpec for SPI_I2SCFGRrs {
    type Ux = u16;
}
///`read()` method returns [`spi_i2scfgr::R`](R) reader structure
impl crate::Readable for SPI_I2SCFGRrs {}
///`write(|w| ..)` method takes [`spi_i2scfgr::W`](W) writer structure
impl crate::Writable for SPI_I2SCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPI_I2SCFGR to value 0
impl crate::Resettable for SPI_I2SCFGRrs {}
