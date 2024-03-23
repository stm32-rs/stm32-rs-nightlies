#[doc = "Register `GTZC1_MPCBB1_PRIVCFGR22` reader"]
pub type R = crate::R<GTZC1_MPCBB1_PRIVCFGR22rs>;
#[doc = "Register `GTZC1_MPCBB1_PRIVCFGR22` writer"]
pub type W = crate::W<GTZC1_MPCBB1_PRIVCFGR22rs>;
#[doc = "Field `PRIV0` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV0_R = crate::BitReader;
#[doc = "Field `PRIV0` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV1` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV1_R = crate::BitReader;
#[doc = "Field `PRIV1` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV2` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV2_R = crate::BitReader;
#[doc = "Field `PRIV2` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV3` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV3_R = crate::BitReader;
#[doc = "Field `PRIV3` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV4` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV4_R = crate::BitReader;
#[doc = "Field `PRIV4` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV5` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV5_R = crate::BitReader;
#[doc = "Field `PRIV5` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV6` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV6_R = crate::BitReader;
#[doc = "Field `PRIV6` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV7` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV7_R = crate::BitReader;
#[doc = "Field `PRIV7` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV8` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV8_R = crate::BitReader;
#[doc = "Field `PRIV8` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV9` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV9_R = crate::BitReader;
#[doc = "Field `PRIV9` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV10` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV10_R = crate::BitReader;
#[doc = "Field `PRIV10` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV11` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV11_R = crate::BitReader;
#[doc = "Field `PRIV11` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV12` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV12_R = crate::BitReader;
#[doc = "Field `PRIV12` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV13` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV13_R = crate::BitReader;
#[doc = "Field `PRIV13` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV14` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV14_R = crate::BitReader;
#[doc = "Field `PRIV14` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV15` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV15_R = crate::BitReader;
#[doc = "Field `PRIV15` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV16` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV16_R = crate::BitReader;
#[doc = "Field `PRIV16` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV17` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV17_R = crate::BitReader;
#[doc = "Field `PRIV17` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV17_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV18` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV18_R = crate::BitReader;
#[doc = "Field `PRIV18` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV18_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV19` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV19_R = crate::BitReader;
#[doc = "Field `PRIV19` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV19_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV20` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV20_R = crate::BitReader;
#[doc = "Field `PRIV20` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV20_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV21` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV21_R = crate::BitReader;
#[doc = "Field `PRIV21` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV21_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV22` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV22_R = crate::BitReader;
#[doc = "Field `PRIV22` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV22_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV23` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV23_R = crate::BitReader;
#[doc = "Field `PRIV23` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV23_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV24` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV24_R = crate::BitReader;
#[doc = "Field `PRIV24` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV24_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV25` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV25_R = crate::BitReader;
#[doc = "Field `PRIV25` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV25_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV26` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV26_R = crate::BitReader;
#[doc = "Field `PRIV26` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV26_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV27` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV27_R = crate::BitReader;
#[doc = "Field `PRIV27` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV27_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV28` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV28_R = crate::BitReader;
#[doc = "Field `PRIV28` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV28_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV29` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV29_R = crate::BitReader;
#[doc = "Field `PRIV29` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV29_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV30` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV30_R = crate::BitReader;
#[doc = "Field `PRIV30` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV30_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV31` reader - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV31_R = crate::BitReader;
#[doc = "Field `PRIV31` writer - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
pub type PRIV31_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv0(&self) -> PRIV0_R {
        PRIV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv1(&self) -> PRIV1_R {
        PRIV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv2(&self) -> PRIV2_R {
        PRIV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv3(&self) -> PRIV3_R {
        PRIV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv4(&self) -> PRIV4_R {
        PRIV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv5(&self) -> PRIV5_R {
        PRIV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv6(&self) -> PRIV6_R {
        PRIV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv7(&self) -> PRIV7_R {
        PRIV7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv8(&self) -> PRIV8_R {
        PRIV8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv9(&self) -> PRIV9_R {
        PRIV9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv10(&self) -> PRIV10_R {
        PRIV10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv11(&self) -> PRIV11_R {
        PRIV11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv12(&self) -> PRIV12_R {
        PRIV12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv13(&self) -> PRIV13_R {
        PRIV13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv14(&self) -> PRIV14_R {
        PRIV14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv15(&self) -> PRIV15_R {
        PRIV15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv16(&self) -> PRIV16_R {
        PRIV16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv17(&self) -> PRIV17_R {
        PRIV17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv18(&self) -> PRIV18_R {
        PRIV18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv19(&self) -> PRIV19_R {
        PRIV19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv20(&self) -> PRIV20_R {
        PRIV20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv21(&self) -> PRIV21_R {
        PRIV21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv22(&self) -> PRIV22_R {
        PRIV22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv23(&self) -> PRIV23_R {
        PRIV23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv24(&self) -> PRIV24_R {
        PRIV24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv25(&self) -> PRIV25_R {
        PRIV25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv26(&self) -> PRIV26_R {
        PRIV26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv27(&self) -> PRIV27_R {
        PRIV27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv28(&self) -> PRIV28_R {
        PRIV28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv29(&self) -> PRIV29_R {
        PRIV29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv30(&self) -> PRIV30_R {
        PRIV30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    pub fn priv31(&self) -> PRIV31_R {
        PRIV31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv0(&mut self) -> PRIV0_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv1(&mut self) -> PRIV1_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv2(&mut self) -> PRIV2_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv3(&mut self) -> PRIV3_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv4(&mut self) -> PRIV4_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv5(&mut self) -> PRIV5_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv6(&mut self) -> PRIV6_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv7(&mut self) -> PRIV7_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv8(&mut self) -> PRIV8_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv9(&mut self) -> PRIV9_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv10(&mut self) -> PRIV10_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv11(&mut self) -> PRIV11_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv12(&mut self) -> PRIV12_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv13(&mut self) -> PRIV13_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv14(&mut self) -> PRIV14_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv15(&mut self) -> PRIV15_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv16(&mut self) -> PRIV16_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv17(&mut self) -> PRIV17_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV17_W::new(self, 17)
    }
    #[doc = "Bit 18 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv18(&mut self) -> PRIV18_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV18_W::new(self, 18)
    }
    #[doc = "Bit 19 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv19(&mut self) -> PRIV19_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV19_W::new(self, 19)
    }
    #[doc = "Bit 20 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv20(&mut self) -> PRIV20_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV20_W::new(self, 20)
    }
    #[doc = "Bit 21 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv21(&mut self) -> PRIV21_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV21_W::new(self, 21)
    }
    #[doc = "Bit 22 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv22(&mut self) -> PRIV22_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV22_W::new(self, 22)
    }
    #[doc = "Bit 23 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv23(&mut self) -> PRIV23_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV23_W::new(self, 23)
    }
    #[doc = "Bit 24 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv24(&mut self) -> PRIV24_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV24_W::new(self, 24)
    }
    #[doc = "Bit 25 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv25(&mut self) -> PRIV25_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV25_W::new(self, 25)
    }
    #[doc = "Bit 26 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv26(&mut self) -> PRIV26_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV26_W::new(self, 26)
    }
    #[doc = "Bit 27 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv27(&mut self) -> PRIV27_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV27_W::new(self, 27)
    }
    #[doc = "Bit 28 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv28(&mut self) -> PRIV28_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV28_W::new(self, 28)
    }
    #[doc = "Bit 29 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv29(&mut self) -> PRIV29_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV29_W::new(self, 29)
    }
    #[doc = "Bit 30 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv30(&mut self) -> PRIV30_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV30_W::new(self, 30)
    }
    #[doc = "Bit 31 - Privileged configuration for block y, belonging to super-block x (y = 31 to 0). Non-secure write to this bit is ignored if SECy bit is set in GTZC1_MPCBBz_SECCFGRx. Writes are ignored if SPLCKx bit is set in GTZC1_MPCBBz_CFGLOCK."]
    #[inline(always)]
    #[must_use]
    pub fn priv31(&mut self) -> PRIV31_W<GTZC1_MPCBB1_PRIVCFGR22rs> {
        PRIV31_W::new(self, 31)
    }
}
#[doc = "GTZC1 SRAM1 MPCBB privileged configuration for super-block 22 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtzc1_mpcbb1_privcfgr22::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtzc1_mpcbb1_privcfgr22::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GTZC1_MPCBB1_PRIVCFGR22rs;
impl crate::RegisterSpec for GTZC1_MPCBB1_PRIVCFGR22rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtzc1_mpcbb1_privcfgr22::R`](R) reader structure"]
impl crate::Readable for GTZC1_MPCBB1_PRIVCFGR22rs {}
#[doc = "`write(|w| ..)` method takes [`gtzc1_mpcbb1_privcfgr22::W`](W) writer structure"]
impl crate::Writable for GTZC1_MPCBB1_PRIVCFGR22rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GTZC1_MPCBB1_PRIVCFGR22 to value 0xffff_ffff"]
impl crate::Resettable for GTZC1_MPCBB1_PRIVCFGR22rs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
